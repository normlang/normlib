use std::{
    fs::File,
    io::{self, BufReader, BufWriter, Read, Write},
    path::Path,
};

use crate::{core::Id16, file::codec::ser::Serialize};

pub trait Payload: Sized {
    const MAGIC: [u8; 8];
    const VERSION: u16;

    fn serialize(&self) -> Vec<u8>;
    fn deserialize(version: u16, bytes: &[u8]) -> io::Result<Self>;
}

pub struct Metadata<P: Payload> {
    last_id: Id16,
    payload: P,
}

impl<P: Payload> Metadata<P> {
    pub const NAME: &'static str = "metadata";
    pub const VERSION: u16 = 1;

    pub fn new(payload: P) -> Self {
        Self {
            last_id: Id16::default(),
            payload,
        }
    }

    pub fn write(&self, path: &Path) -> io::Result<()> {
        let mut metadata_path = path.to_path_buf();
        metadata_path.push(Self::NAME);

        let mut buf: Vec<u8> = Vec::new();
        Self::VERSION.serialize(&mut buf);
        self.last_id.serialize(&mut buf);
        P::VERSION.serialize(&mut buf);

        let mut w = BufWriter::new(File::create(metadata_path)?);
        w.write_all(&P::MAGIC)?;
        w.write_all(&buf)?;
        w.write_all(&self.payload.serialize())?;
        w.flush()?;

        Ok(())
    }

    pub fn read(path: &Path) -> io::Result<Self> {
        let mut metadata_path = path.to_path_buf();
        metadata_path.push(Self::NAME);

        let mut reader = BufReader::new(File::open(metadata_path)?);

        let mut magic = [0u8; 8];
        reader.read_exact(&mut magic)?;

        if magic != P::MAGIC {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "wrong metadata magic",
            ));
        }

        let mut version_buf = [0u8; size_of::<u16>()];
        reader.read_exact(&mut version_buf)?;
        let version = u16::from_le_bytes(version_buf);

        if version != Self::VERSION {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "wrong metadata version",
            ));
        }

        let mut last_id_buf = [0u8; size_of::<Id16>()];
        reader.read_exact(&mut last_id_buf)?;
        let last_id = Id16::new(u16::from_le_bytes(last_id_buf));

        reader.read_exact(&mut version_buf)?;
        let payload_version = u16::from_le_bytes(version_buf);

        let mut payload_buf = Vec::new();
        reader.read_to_end(&mut payload_buf)?;
        let payload = P::deserialize(payload_version, &payload_buf)?;

        Ok(Self { last_id, payload })
    }

    pub fn payload(&self) -> &P {
        &self.payload
    }

    pub fn payload_mut(&mut self) -> &mut P {
        &mut self.payload
    }

    pub fn next_id(&mut self) -> Option<Id16> {
        self.last_id = self.last_id.increment()?;
        Some(self.last_id)
    }
}

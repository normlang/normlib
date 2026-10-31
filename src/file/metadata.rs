use std::{
    fs::File,
    io::{self, BufReader, BufWriter, Read, Write},
    path::Path,
};

use crate::core::Id;

pub trait Payload: Sized {
    const MAGIC: [u8; 8];
    const VERSION: u16;

    fn serialize(&self) -> &[u8];
    fn deserialize(version: u16, bytes: &[u8]) -> io::Result<Self>;
}

pub struct Metadata<P: Payload> {
    last_id: Id,
    payload: P,
}

impl<P: Payload> Metadata<P> {
    pub const NAME: &'static str = "metadata";
    pub const VERSION: u16 = 1;

    pub fn new(payload: P) -> Self {
        Self {
            last_id: Id::default(),
            payload,
        }
    }

    pub fn write(&self, path: &Path) -> io::Result<()> {
        let mut metadata_path = path.to_path_buf();
        metadata_path.push(Self::NAME);

        let mut w = BufWriter::new(File::create(metadata_path)?);
        w.write_all(&P::MAGIC)?;
        w.write_all(&Self::VERSION.to_le_bytes())?;
        w.write_all(&self.last_id.get().to_le_bytes())?;
        w.write_all(&P::VERSION.to_le_bytes())?;
        w.write_all(&self.payload.serialize())?;
        w.flush()?;

        Ok(())
    }

    pub fn read(path: &Path) -> io::Result<Self> {
        let mut metadata_path = path.to_path_buf();
        metadata_path.push(Self::NAME);

        let mut r = BufReader::new(File::open(metadata_path)?);

        let mut magic = [0u8; 8];
        r.read_exact(&mut magic)?;

        if magic != P::MAGIC {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "wrong metadata magic",
            ));
        }

        let mut b2 = [0u8; 2];
        r.read_exact(&mut b2)?;
        let version = u16::from_le_bytes(b2);

        if version != Self::VERSION {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "wrong metadata version",
            ));
        }

        let mut b8 = [0u8; 8];
        r.read_exact(&mut b8)?;
        let last_id = Id::new(u64::from_le_bytes(b8));

        r.read_exact(&mut b2)?;
        let payload_version = u16::from_le_bytes(b2);

        let mut bytes = Vec::new();
        r.read_to_end(&mut bytes)?;
        let payload = P::deserialize(payload_version, &bytes)?;

        Ok(Self { last_id, payload })
    }

    pub fn payload(&self) -> &P {
        &self.payload
    }

    pub fn payload_mut(&mut self) -> &mut P {
        &mut self.payload
    }

    pub fn next_id(&mut self) -> Id {
        self.last_id = self.last_id.increment();
        self.last_id
    }
}

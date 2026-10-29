use std::{
    fs::File,
    io::{self, BufWriter, Write},
    path::Path,
};

use crate::core::Id;

pub trait Payload {
    fn version(&self) -> u16;
    fn magic(&self) -> [u8; 8];
    fn serialize(&self) -> &[u8];
    fn deserialize(&self, bytes: &[u8]);
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
        w.write_all(&self.payload.magic())?;
        w.write_all(&Self::VERSION.to_le_bytes())?;
        w.write_all(&self.last_id.get().to_le_bytes())?;
        w.write_all(&self.payload.version().to_le_bytes())?;
        w.write(&self.payload.serialize())?;
        w.flush()?;

        Ok(())
    }
}

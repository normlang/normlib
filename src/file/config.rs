use std::{
    fs::{self, File},
    io::{self, BufWriter, Write},
    path::{Path, PathBuf},
};

use crate::{core::Id, file::normspace::Normspace};

pub struct Config {
    _path: PathBuf,
    metadata: Metadata,
}

impl Config {
    pub const DIR_NAME: &'static str = "norm";

    pub fn create(path: Option<&Path>) -> io::Result<Self> {
        let config_dir = Self::create_config_dir(path)?;
        let config = Self::new(&config_dir)?;
        Ok(config)
    }

    fn create_config_dir(path: Option<&Path>) -> io::Result<PathBuf> {
        let user_config_dir = match path {
            Some(p) => p.to_path_buf(),
            None => dirs::config_dir().ok_or_else(|| {
                io::Error::new(io::ErrorKind::NotFound, "user config dir not found")
            })?,
        };

        let norm_config_dir = user_config_dir.join(Self::DIR_NAME);
        fs::create_dir(&norm_config_dir)?;
        Ok(norm_config_dir)
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    pub fn new(path: &Path) -> io::Result<Self> {
        Ok(Self {
            _path: path.to_path_buf(),
            metadata: Metadata::new(),
        })
    }
}

pub struct Metadata {
    _normspaces: Vec<Normspace>,
    _last_normspace_id: Id,
}

impl Metadata {
    pub const NAME: &'static str = "metadata";
    pub const MAGIC: &[u8; 8] = b"\x5a\xe1\x0c\x93\x7f\xd4\x28\xb6";
    pub const VERSION: u16 = 1;

    pub fn new() -> Self {
        Self {
            _normspaces: Vec::new(),
            _last_normspace_id: Id::default(),
        }
    }

    pub fn write(&self, path: &Path) -> io::Result<()> {
        let mut metadata_path = path.to_path_buf();
        metadata_path.push(Self::NAME);

        let mut w = BufWriter::new(File::create(metadata_path)?);
        w.write_all(Self::MAGIC)?;
        w.write_all(&Self::VERSION.to_le_bytes())?;
        w.flush()?;

        Ok(())
    }
}

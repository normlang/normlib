use std::{
    fs, io,
    path::{Path, PathBuf},
};

use crate::{
    core::Id,
    file::{
        metadata::{Metadata, Payload},
        normspace::Normspace,
    },
};

pub struct Config {
    path: PathBuf,
    metadata: Metadata<ConfigData>,
}

struct ConfigData {
    normspaces: Vec<Normspace>,
    _last_normspace_id: Id,
}

impl Config {
    pub const DIR_NAME: &'static str = "norm";

    pub fn create(path: Option<&Path>) -> io::Result<Self> {
        let config_dir = Self::create_config_dir(path)?;
        let config = Self::new(&config_dir)?;
        config.write()?;
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

    pub fn new(path: &Path) -> io::Result<Self> {
        let data = ConfigData {
            normspaces: Vec::new(),
            _last_normspace_id: Id::default(),
        };

        Ok(Self {
            path: path.to_path_buf(),
            metadata: Metadata::new(data),
        })
    }

    pub fn write(&self) -> io::Result<()> {
        self.metadata.write(&self.path)?;
        Ok(())
    }
}

impl Payload for ConfigData {
    fn version(&self) -> u16 {
        1
    }

    fn magic(&self) -> [u8; 8] {
        [0x5a, 0xe1, 0x0c, 0x93, 0x7f, 0xd4, 0x28, 0xb6]
    }

    fn serialize(&self) -> &[u8] {
        &[]
    }

    fn deserialize(&self, bytes: &[u8]) {
        todo!()
    }
}

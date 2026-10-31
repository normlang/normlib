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
}

impl Config {
    pub const DIR_NAME: &'static str = "norm";

    pub fn new() -> io::Result<Self> {
        let user_config_dir = dirs::config_dir().ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, "invalid user config directory")
        })?;

        Ok(Self::with_path(&user_config_dir)?)
    }

    pub fn with_path(path: &Path) -> io::Result<Self> {
        let norm_config_dir = path.join(Self::DIR_NAME);

        let norm_config = if !norm_config_dir.exists() {
            fs::create_dir(&norm_config_dir)?;
            let data = ConfigData {
                normspaces: Vec::new(),
            };
            let config = Config {
                path: norm_config_dir,
                metadata: Metadata::new(data),
            };
            config.write()?;
            config
        } else {
            Self::read_from_path(&norm_config_dir)?
        };

        Ok(norm_config)
    }

    pub fn write(&self) -> io::Result<()> {
        self.metadata.write(&self.path)?;
        Ok(())
    }

    fn read_from_path(path: &Path) -> io::Result<Self> {
        let metadata = Metadata::read(path)?;
        Ok(Config {
            path: path.to_path_buf(),
            metadata,
        })
    }

    pub fn add_normspace(&mut self, name: &str, dir: &Path) -> io::Result<Id> {
        let path = dir.join(name);
        let id = self.metadata.next_id();
        let normspace = Normspace::with_path(&path, id)?;
        normspace.write()?;
        self.metadata.payload_mut().normspaces.push(normspace);
        self.write()?;
        Ok(id)
    }
}

impl Payload for ConfigData {
    const MAGIC: [u8; 8] = [0x5a, 0xe1, 0x0c, 0x93, 0x7f, 0xd4, 0x28, 0xb6];
    const VERSION: u16 = 1;

    fn serialize(&self) -> &[u8] {
        &[]
    }

    fn deserialize(version: u16, bytes: &[u8]) -> io::Result<Self> {
        Ok(ConfigData {
            normspaces: Vec::new(),
        })
    }
}

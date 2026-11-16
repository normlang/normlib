use std::{
    fs,
    io::{self, Cursor, Read},
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::{
    core::Id16,
    file::{
        metadata::{Metadata, Payload},
        normspace::Normspace,
    },
};

pub struct Config {
    path: PathBuf,
    metadata: Metadata<ConfigMeta>,
}

pub struct ConfigMeta {
    normspaces: Vec<NormspaceMeta>,
}

pub struct NormspaceMeta {
    pub id: Id16,
    pub name: String,
    pub path: PathBuf,
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
            let data = ConfigMeta {
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

    pub fn new_normspace(&mut self, name: &str, dir: &Path) -> io::Result<Id16> {
        let id = self.metadata.next_id().ok_or(io::ErrorKind::InvalidData)?;
        let file_name = format!("ns!{:04X}", id.get());
        let path = dir.join(file_name);
        fs::create_dir(&path)?;

        let normspace = Normspace::with_path(&path)?;
        normspace.write()?;

        let normspace_meta = NormspaceMeta {
            id,
            name: String::from(name),
            path,
        };

        self.metadata.payload_mut().normspaces.push(normspace_meta);
        self.write()?;

        Ok(id)
    }

    pub fn normspaces(&self) -> &[NormspaceMeta] {
        &self.metadata.payload().normspaces
    }
}

impl Payload for ConfigMeta {
    const MAGIC: [u8; 8] = [0x5a, 0xe1, 0x0c, 0x93, 0x7f, 0xd4, 0x28, 0xb6];
    const VERSION: u16 = 1;

    fn serialize(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        let size = self.normspaces.len() as u16;
        result.append(&mut size.to_le_bytes().to_vec());

        for normspace in self.normspaces.iter() {
            result.append(&mut normspace.id.get().to_le_bytes().to_vec());

            let name_size = normspace.name.len() as u16;
            result.append(&mut name_size.to_le_bytes().to_vec());
            result.append(&mut normspace.name.as_bytes().to_vec());

            let path = normspace.path.to_str().unwrap_or_default();
            let path_size = path.len() as u16;
            result.append(&mut path_size.to_le_bytes().to_vec());
            result.append(&mut path.as_bytes().to_vec());
        }

        result
    }

    fn deserialize(version: u16, bytes: &[u8]) -> io::Result<Self> {
        _ = version;

        let mut cur = Cursor::new(bytes);
        let mut size_buff = [0u8; size_of::<u16>()];
        cur.read_exact(&mut size_buff)?;
        let size = u16::from_le_bytes(size_buff);

        let mut normspaces: Vec<NormspaceMeta> = Vec::new();

        for _ in 0..size {
            let mut id_buf = [0u8; size_of::<Id16>()];
            cur.read_exact(&mut id_buf)?;
            let id = Id16::new(u16::from_be_bytes(id_buf));

            let mut name_size_buf = [0u8; size_of::<u16>()];
            cur.read_exact(&mut name_size_buf)?;
            let name_size = u16::from_be_bytes(name_size_buf);
            let mut name_buf = vec![0u8; name_size as usize];
            cur.read_exact(&mut name_buf)?;
            let name = String::from_utf8(name_buf)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

            let mut path_size_buf = [0u8; size_of::<u16>()];
            cur.read_exact(&mut path_size_buf)?;
            let path_size = u16::from_be_bytes(path_size_buf);
            let mut path_buf = vec![0u8; path_size as usize];
            cur.read_exact(&mut path_buf)?;
            let path_str = String::from_utf8(path_buf)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
            let path = PathBuf::from_str(&path_str).unwrap_or_default();

            let normspace = NormspaceMeta { id, name, path };
            normspaces.push(normspace);
        }

        Ok(ConfigMeta { normspaces })
    }
}

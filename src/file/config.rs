use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{core::Id, file::normspace::Normspace};

pub struct Config {
    _path: PathBuf,
    _normspaces: Vec<Normspace>,
    _last_normspace_id: Id,
}

impl Config {
    pub const DIR_NAME: &'static str = "norm";

    pub fn create(path: Option<&Path>) -> std::io::Result<Self> {
        let config_dir = Self::create_config_dir(path)?;
        let config = Self::new(&config_dir)?;
        Ok(config)
    }

    fn create_config_dir(path: Option<&Path>) -> std::io::Result<PathBuf> {
        let user_config_dir = match path {
            Some(p) => p.to_path_buf(),
            None => dirs::config_dir().ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::NotFound, "user config dir not found")
            })?,
        };

        let norm_config_dir = user_config_dir.join(Self::DIR_NAME);
        fs::create_dir(&norm_config_dir)?;
        Ok(norm_config_dir)
    }

    pub fn new(path: &Path) -> std::io::Result<Self> {
        Ok(Self {
            _path: path.to_path_buf(),
            _normspaces: Vec::new(),
            _last_normspace_id: Id::default(),
        })
    }
}

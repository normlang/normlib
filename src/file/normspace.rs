use std::{
    fs,
    path::{Path, PathBuf},
};

pub const DOT_DIR_NAME: &'static str = ".normspace";

pub struct Normspace {
    metadata: Metadata,
}

impl Normspace {
    pub fn new(path: &Path) -> Result<Self, std::io::Error> {
        let dot_dir_path: PathBuf = path.join(DOT_DIR_NAME);
        fs::create_dir(dot_dir_path.as_path())?;

        Ok(Self {
            metadata: Metadata::new(dot_dir_path.as_path())?,
        })
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

pub struct Metadata {}

impl Metadata {
    pub fn new(path: &Path) -> Result<Self, std::io::Error> {
        let bytes: Vec<u8> = Vec::new();
        fs::write(path, bytes)?;
        Ok(Metadata {})
    }
}

pub fn create(name: &str) -> std::io::Result<()> {
    fs::create_dir(name)?;

    let dot_dir_path = String::from(name) + "/" + DOT_DIR_NAME;
    fs::create_dir(dot_dir_path)?;

    Ok(())
}

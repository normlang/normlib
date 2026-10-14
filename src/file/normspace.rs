use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct Normspace {
    metadata: Metadata,
}

impl Normspace {
    pub fn new(path: &Path) -> Result<Self, std::io::Error> {
        let dot_normspace: PathBuf = path.join(".normspace");
        fs::create_dir(dot_normspace.as_path())?;

        Ok(Self {
            metadata: Metadata::new(dot_normspace.as_path())?,
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

    let dot_normspace = String::from(name) + "/.normspace";
    fs::create_dir(dot_normspace)?;

    Ok(())
}

use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct Workspace {
    metadata: Metadata,
}

impl Workspace {
    pub fn new(path: &Path) -> Result<Self, std::io::Error> {
        let dot_workspace: PathBuf = path.join(".workspace");
        fs::create_dir(dot_workspace.as_path())?;

        Ok(Self {
            metadata: Metadata::new(dot_workspace.as_path())?,
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

    let dot_workspace = String::from(name) + "/.workspace";
    fs::create_dir(dot_workspace)?;

    Ok(())
}

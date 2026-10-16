use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct Workspace {
    path: PathBuf,
    metadata: Metadata,
}

impl Workspace {
    pub const DOT_DIR_NAME: &'static str = ".workspace";

    pub fn create(path: impl AsRef<Path>) -> std::io::Result<Workspace> {
        let root = path.as_ref();
        fs::create_dir(root)?;

        let dot_dir_path = root.join(Workspace::DOT_DIR_NAME);
        fs::create_dir(&dot_dir_path)?;

        let workspace = Workspace::new(&dot_dir_path)?;
        workspace.save()?;

        Ok(workspace)
    }

    pub fn new(path: &Path) -> Result<Self, std::io::Error> {
        Ok(Self {
            path: path.to_path_buf(),
            metadata: Metadata::new(),
        })
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        self.metadata.save(&self.path)?;
        Ok(())
    }
}

pub struct Metadata {}

impl Metadata {
    pub const NAME: &'static str = "metadata";

    pub fn new() -> Self {
        Metadata {}
    }

    pub fn save(&self, path: &Path) -> Result<(), std::io::Error> {
        let bytes: Vec<u8> = Vec::new();
        let mut metadata_path = path.to_path_buf();
        metadata_path.push(Metadata::NAME);
        fs::write(metadata_path.as_path(), bytes)?;
        Ok(())
    }
}

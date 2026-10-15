use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct Normspace {
    path: PathBuf,
    metadata: Metadata,
}

impl Normspace {
    pub const DOT_DIR_NAME: &'static str = ".normspace";

    pub fn create(path: impl AsRef<Path>) -> std::io::Result<Normspace> {
        let root = path.as_ref();
        fs::create_dir(root)?;

        let dot_dir_path = root.join(Normspace::DOT_DIR_NAME);
        fs::create_dir(&dot_dir_path)?;

        let normspace = Normspace::new(&dot_dir_path)?;
        normspace.save()?;

        Ok(normspace)
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

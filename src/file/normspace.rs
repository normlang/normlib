use std::{
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};

pub struct Normspace {
    path: PathBuf,
    metadata: Metadata,
}

impl Normspace {
    pub const DOT_DIR_NAME: &'static str = ".normspace";

    pub fn create(name: &str) -> std::io::Result<Normspace> {
        fs::create_dir(name)?;

        let dot_dir_path = String::from(name) + "/" + Normspace::DOT_DIR_NAME;
        fs::create_dir(&dot_dir_path)?;

        let path = PathBuf::from_str(&dot_dir_path).expect("normspace path should be exists");
        let normspace = Normspace::new(path.as_path())?;
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

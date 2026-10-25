use std::{
    fs::{self, File},
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

use crate::file::config::Config;

pub struct Normspace {
    path: PathBuf,
    metadata: Metadata,
}

impl Normspace {
    pub const DOT_DIR_NAME: &'static str = ".normspace";

    pub fn create(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let _ = Config::create(None);

        let root = path.as_ref();
        fs::create_dir(root)?;

        let dot_dir_path = root.join(Self::DOT_DIR_NAME);
        fs::create_dir(&dot_dir_path)?;

        let normspace = Self::new(&dot_dir_path)?;
        normspace.write()?;

        Ok(normspace)
    }

    pub fn new(path: &Path) -> std::io::Result<Self> {
        Ok(Self {
            path: path.to_path_buf(),
            metadata: Metadata::new(),
        })
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    pub fn write(&self) -> std::io::Result<()> {
        self.metadata.write(&self.path)?;
        Ok(())
    }
}

pub struct Metadata {}

impl Metadata {
    pub const NAME: &'static str = "metadata";
    pub const MAGIC: &[u8; 8] = b"\xb3\xff\xca\x01\xf3\xc7\x64\x9d";
    pub const VERSION: u16 = 1;

    pub fn new() -> Self {
        Self {}
    }

    pub fn write(&self, path: &Path) -> std::io::Result<()> {
        let mut metadata_path = path.to_path_buf();
        metadata_path.push(Self::NAME);

        let mut w = BufWriter::new(File::create(metadata_path)?);
        w.write_all(Self::MAGIC)?;
        w.write_all(&Self::VERSION.to_le_bytes())?;
        w.flush()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn create_normspace() {
        let tmp_dir = tempdir().unwrap();
        let root = tmp_dir.path().join("ns");
        let normspace = Normspace::create(&root).unwrap();
        let dot_dir = root.join(Normspace::DOT_DIR_NAME);

        assert!(root.is_dir());
        assert!(dot_dir.is_dir());
        assert_eq!(normspace.path, dot_dir);

        let meta_file = dot_dir.join(Metadata::NAME);
        assert!(meta_file.is_file());
    }
}

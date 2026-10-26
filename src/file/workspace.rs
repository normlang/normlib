use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub struct Workspace {
    path: PathBuf,
    metadata: Metadata,
}

impl Workspace {
    pub const DOT_DIR_NAME: &'static str = ".workspace";

    pub fn create(path: impl AsRef<Path>) -> io::Result<Self> {
        let root = path.as_ref();
        fs::create_dir(root)?;

        let dot_dir_path = root.join(Self::DOT_DIR_NAME);
        fs::create_dir(&dot_dir_path)?;

        let workspace = Self::new(&dot_dir_path)?;
        workspace.write()?;

        Ok(workspace)
    }

    pub fn new(path: &Path) -> io::Result<Self> {
        Ok(Self {
            path: path.to_path_buf(),
            metadata: Metadata::new(),
        })
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    pub fn write(&self) -> io::Result<()> {
        self.metadata.write(&self.path)?;
        Ok(())
    }
}

pub struct Metadata {}

impl Metadata {
    pub const NAME: &'static str = "metadata";

    pub fn new() -> Self {
        Self {}
    }

    pub fn write(&self, path: &Path) -> io::Result<()> {
        let bytes: Vec<u8> = Vec::new();
        let mut metadata_path = path.to_path_buf();
        metadata_path.push(Self::NAME);
        fs::write(metadata_path.as_path(), bytes)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn create_workspace() {
        let tmp_dir = tempdir().unwrap();
        let root = tmp_dir.path().join("ws");

        let workspace = Workspace::create(&root).unwrap();
        let dot_dir = root.join(Workspace::DOT_DIR_NAME);

        assert!(root.is_dir());
        assert!(dot_dir.is_dir());
        assert_eq!(workspace.path, dot_dir);

        let meta_file = dot_dir.join(Metadata::NAME);
        assert!(meta_file.is_file());
    }
}

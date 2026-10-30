use std::{
    fs, io,
    path::{Path, PathBuf},
};

use crate::{
    core::Id,
    file::{
        config::Config,
        metadata::{Metadata, Payload},
        workspace::Workspace,
    },
};

pub struct Normspace {
    path: PathBuf,
    metadata: Metadata<NormspaceData>,
}

struct NormspaceData {
    workspaces: Vec<Workspace>,
}

impl Normspace {
    pub const DOT_DIR_NAME: &'static str = ".normspace";

    pub fn create(path: impl AsRef<Path>) -> io::Result<Self> {
        let _ = Config::create(None);

        let root = path.as_ref();
        fs::create_dir(root)?;

        let dot_dir_path = root.join(Self::DOT_DIR_NAME);
        fs::create_dir(&dot_dir_path)?;

        let normspace = Self::new(&dot_dir_path)?;
        normspace.write()?;

        Ok(normspace)
    }

    pub fn new(path: &Path) -> io::Result<Self> {
        let data = NormspaceData {
            workspaces: Vec::new(),
        };

        Ok(Self {
            path: path.to_path_buf(),
            metadata: Metadata::new(data),
        })
    }

    pub fn write(&self) -> io::Result<()> {
        self.metadata.write(&self.path)?;
        Ok(())
    }
}

impl Payload for NormspaceData {
    const MAGIC: [u8; 8] = [0xb3, 0xff, 0xca, 0x01, 0xf3, 0xc7, 0x64, 0x9d];

    fn version(&self) -> u16 {
        1
    }

    fn serialize(&self) -> &[u8] {
        &[]
    }

    fn deserialize(&self, bytes: &[u8]) {
        todo!()
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
    }
}

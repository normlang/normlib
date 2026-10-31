use std::{
    env, fs, io,
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
    id: Id,
    workspaces: Vec<Workspace>,
}

impl Normspace {
    pub const DOT_DIR_NAME: &'static str = ".normspace";

    pub fn add(name: &str) -> io::Result<Id> {
        let mut config = Config::new()?;
        let dir = env::current_dir()?;
        config.add_normspace(name, &dir)
    }

    pub fn with_path(path: &Path, id: Id) -> io::Result<Self> {
        fs::create_dir(path)?;

        let dot_dir_path = path.join(Self::DOT_DIR_NAME);
        fs::create_dir(&dot_dir_path)?;

        let data = NormspaceData {
            id,
            workspaces: Vec::new(),
        };

        Ok(Normspace {
            path: dot_dir_path,
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
    const VERSION: u16 = 1;

    fn serialize(&self) -> &[u8] {
        &[]
    }

    fn deserialize(version: u16, bytes: &[u8]) -> io::Result<Self> {
        Ok(NormspaceData {
            id: Id::default(),
            workspaces: Vec::new(),
        })
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
        let normspace = Normspace::with_path(&root, Id::new(42)).unwrap();
        let dot_dir = root.join(Normspace::DOT_DIR_NAME);

        assert!(root.is_dir());
        assert!(dot_dir.is_dir());
        assert_eq!(normspace.path, root);
    }
}

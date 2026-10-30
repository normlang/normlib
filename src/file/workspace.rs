use std::{
    fs, io,
    path::{Path, PathBuf},
};

use crate::{
    core::Id,
    file::{
        metadata::{Metadata, Payload},
        project::Project,
    },
};

pub struct Workspace {
    path: PathBuf,
    metadata: Metadata<WorkspaceData>,
}

struct WorkspaceData {
    projects: Vec<Project>,
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
        let data = WorkspaceData {
            projects: Vec::new(),
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

impl Payload for WorkspaceData {
    const MAGIC: [u8; 8] = [0xb3, 0x2f, 0x8b, 0xd1, 0x46, 0xa0, 0x73, 0x1c];

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
    fn create_workspace() {
        let tmp_dir = tempdir().unwrap();
        let root = tmp_dir.path().join("ws");

        let workspace = Workspace::create(&root).unwrap();
        let dot_dir = root.join(Workspace::DOT_DIR_NAME);

        assert!(root.is_dir());
        assert!(dot_dir.is_dir());
        assert_eq!(workspace.path, dot_dir);
    }
}

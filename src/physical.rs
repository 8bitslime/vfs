use crate::Result;
use crate::{VFSFile, VFSMount};
use std::path::{Path, PathBuf};
use std::io::{Error, ErrorKind};
use std::fs::File;

impl VFSFile for File {}

#[derive(Debug)]
pub struct PhysicalMount {
    path: PathBuf
}

impl PhysicalMount {
    pub fn new(path: &Path) -> Result<Self> {
        let path = path.to_owned().canonicalize()?;
        if path.is_dir() {
            Ok(Self { path })
        } else {
            Err(Error::new(ErrorKind::NotFound, "No such directory"))
        }
    }
}

impl VFSMount for PhysicalMount {
    fn open(&self, path: &str) -> Result<Box<dyn VFSFile>> {
        let joined_path = self.path.join(path).canonicalize()?;
        File::open(joined_path).map(|f| Box::new(f) as Box<dyn VFSFile>)
    }
}

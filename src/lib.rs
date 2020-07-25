pub use std::io::Result;
use std::io::{Error, ErrorKind};
use std::io::{Read, Write, Seek};
use std::path::Path;

pub mod physical;

#[test]
fn test() {
    let mut vfs = VFS::new();
    vfs.mount_folder("/really/long/path", Path::new("src")).unwrap();
    let mut file = vfs.open("really/long/path/lib.rs").unwrap();
    let mut string = String::new();
    file.read_to_string(&mut string).unwrap();
    println!("{}", string.lines().next().unwrap());
    println!("{:?}", vfs);
}

pub trait VFSFile: std::fmt::Debug + Read + Write + Seek {}

pub trait VFSMount: std::fmt::Debug {
    fn open(&self, path: &str) -> Result<Box<dyn VFSFile>>;
}

#[derive(Debug, Default)]
pub struct VFS {
    mounts: Vec<Mount>
}
impl VFS {
    pub fn new() -> Self { Default::default() }
    
    pub fn mount_folder(&mut self, target: &str, path: &Path) -> Result<()> {
        self.mount(target, Box::new(physical::PhysicalMount::new(path)?));
        Ok(())
    }
    
    pub fn mount(&mut self, target: &str, mount: Box<dyn VFSMount>) {
        let mut tree = target.split('/').filter(|s| !s.is_empty()).collect::<Vec<&str>>();
        
        if let Some(mut name) = tree.pop() {
            let mut prev_mount = mount;
            while let Some(folder) = tree.pop() {
                let mut vfs = Box::new(VFS::new());
                vfs.mount(name, prev_mount);
                prev_mount = vfs;
                name = folder;
            }
            self.mounts.insert(0, Mount {
                name: name.to_owned(),
                mount: prev_mount
            });
        }
    }
}
impl VFSMount for VFS {
    fn open(&self, path: &str) -> Result<Box<dyn VFSFile>> {
        let mut split = path.split('/');
        let folder = split.next().unwrap_or("");
        let path = split.collect::<Vec<&str>>().join("/");
        for mount in &self.mounts {
            if folder == mount.name {
                return mount.mount.open(&path);
            }
        };
        Err(Error::new(ErrorKind::NotFound, format!("No such mounting point: {}", folder)))
    }
}

#[derive(Debug)]
struct Mount {
    name: String,
    mount: Box<dyn VFSMount>
}

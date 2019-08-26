use std::{fs, io};
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::path::PathBuf;
use std::sync::Mutex;



pub struct DedupeFileSet {
    files: HashMap<Vec<u8>, PathBuf>,
}

impl DedupeFileSet {
    pub fn new() -> Self {
        Self {
            files: HashMap::new()
        }
    }

    pub fn commit(&mut self, buffer: Vec<u8>, path: PathBuf) -> io::Result<&PathBuf> {
        // Designed to, hopefully, avoid any extra copies of buffer or path.
        match self.files.entry(buffer) {
            Entry::Occupied(entry) => {
                Ok(entry.into_mut()) // Doesn't need to be mut, but *does* need the 'a lifetime of into_mut
            },
            Entry::Vacant(entry) => {
                let buffer = entry.key(); // buffer was moved
                if path.exists() {
                    // XXX: Make it configurable if we trust the file size alone for "this file changed" checks.
                    let meta = path.metadata()?;
                    if meta.len() == buffer.len() as u64 {
                        return Ok(entry.insert(path));
                    }
                }
                fs::write(&path, buffer)?;
                Ok(entry.insert(path))
            },
        }
    }
}



pub struct ConcurrentDedupeFileSet {
    file_set: Mutex<DedupeFileSet>,
}

impl ConcurrentDedupeFileSet {
    pub fn new() -> Self { Self { file_set: Mutex::new(DedupeFileSet::new()) }}

    pub fn commit(&self, buffer: Vec<u8>, path: PathBuf) -> io::Result<PathBuf> {
        // Unfortunately, we have to clone the PathBuf here as another thread may start modifying self.
        self.file_set.lock().unwrap().commit(buffer, path).map(|p| p.clone())
    }
}

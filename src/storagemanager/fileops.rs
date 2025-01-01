// mod storagemanager::fileops;
// src/storagemanager/fileops.rs
// This module contains the implementation of the file operations for the storage manager.

// There are two types of files in the storage manager:
// 1. Small files that can fit in memory
// 2. Large files that are stored on disk and should be read through a buffer pool (not implemented yet).
// The ManagedFile struct is a wrapper for both types of files. It implements the SmallFile trait for small files, and the LargeFile trait for large files. Dependending on the file size, the storage manager will choose the appropriate file type.

use std::fs::File;
use std::io::{Read, Write, Result, Error, ErrorKind};


// Trait for reading and writing small files that can fit in memory
pub trait SmallFile {
    fn read_to_end(&self) -> Result<Vec<u8>>;
    fn write_all(&self, buf: &[u8]) -> Result<()>;
}


// Struct of a file that can be read and written by the storage manager
#[derive(Debug)]
pub struct ManagedFile {
    path: String,
}

// Implementation of the ManagedFile struct
impl ManagedFile {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }

    fn open_file(&self, mode: &str) -> Result<File> {
        match mode {
            "r" => File::open(&self.path),
            "w" => File::create(&self.path),
            _ => Err(Error::new(ErrorKind::InvalidInput, "Invalid mode")),
        }
    }
}


// Implementation of the SmallFile trait for ManagedFile
impl SmallFile for ManagedFile {
    fn read_to_end(&self) -> Result<Vec<u8>> {
       
        let mut file = self.open_file("r")?;
        // Check if we can fit the file in memory
        assert!(file.metadata()?.len() <= usize::MAX as u64, "File too large to fit in memory");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    fn write_all(&self, buf: &[u8]) -> Result<()> {
       
        // No need to check buf.len() against usize::MAX as it is always true
        let mut file = self.open_file("w")?;
        file.write_all(buf)
    }
}
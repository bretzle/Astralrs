use std::fs::File;
use std::fs::FileType;
use std::fs::metadata;
use std::path::Path;

pub struct Fi {
    file: File,
    file_type: FileType,
}

impl Fi {
    pub fn new(path: &Path) -> Self {
        Self {
            file: File::open(path).unwrap(),
            file_type: metadata(path).unwrap().file_type(),
        }
    }

    pub fn get_file(&self) -> &File {
        &self.file
    }

    pub fn get_type(&self) -> &FileType {
        &self.file_type
    }
}
use std::path::PathBuf;

#[derive(Debug)]
pub enum FileType {
    Directory,
    File,
    Symlink,
}

#[derive(Debug)]
pub struct Item {
    pub file_name: String,
    pub file_path: PathBuf,
    pub file_type: FileType,
    pub file_size: u64,
    pub file_ext: Option<String>,
    pub is_hidden: bool,
    pub is_dirty: bool,
}

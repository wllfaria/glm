use std::path::{Path, PathBuf};

use crate::fs_ops::FsOps;
use crate::list_state::ListState;
use crate::tree_state::TreeState;

/// FileType of a given item, in which can be `Directory | File | Symlink`
#[derive(Debug)]
pub enum FileType {
    Directory,
    File,
    Symlink,
}

/// `Item` is the representation of any contents of the filesystem.
#[derive(Debug)]
pub struct Item {
    /// The basename of the item, the last component on the path
    pub file_name: String,
    /// Absolute path to the item in the filesystem
    pub file_path: PathBuf,
    /// `FileType` of the item,
    pub file_type: FileType,
    /// Extension of the item in `String` format, or `None` for non-file items
    pub file_ext: Option<String>,
    /// Whether this is considered a hidden file or not
    pub is_hidden: bool,
}

#[derive(Debug)]
pub struct FileManager<S> {
    state: S,
}

impl FsOps<ListState> for FileManager<ListState> {
    fn change_dir<P>(&mut self, path: P) -> anyhow::Result<&ListState>
    where
        P: AsRef<Path>,
    {
        let mut items = vec![];
        for entry in std::fs::read_dir(path.as_ref())? {
            let entry = entry?;
            let is_hidden = self.is_hidden(entry.path())?;
            // TODO: we should not just skip hidden files, but rather show/hide
            // based on a dynamic setting
            if is_hidden {
                continue;
            }
            let file_name = entry.file_name().to_string_lossy().to_string();
            let file_path = entry.path();
            let file_type = self.get_file_type(entry.path())?;
            let file_ext = self.get_file_extension(entry.path());
            let item = Item {
                file_name,
                file_path,
                file_type,
                file_ext,
                is_hidden,
            };
            items.push(item);
        }
        let new_state = ListState {
            current_dir: path.as_ref().to_path_buf(),
            items,
        };
        self.state = new_state;
        Ok(&self.state)
    }
}

impl FileManager<ListState> {
    pub fn new<T>(path: T) -> anyhow::Result<FileManager<ListState>>
    where
        T: AsRef<Path>,
    {
        let mut fm = FileManager {
            state: ListState::default(),
        };

        fm.change_dir(path)?;

        Ok(fm)
    }

    /// Returns a immutable reference to the current state. mutating
    /// the state is not directly allowed, in order to achieve mutation
    /// use the specialized methods that mutate the state.
    pub fn get_state(&self) -> &ListState {
        &self.state
    }
}

impl FileManager<TreeState> {
    pub fn new<T>(_: T) -> anyhow::Result<FileManager<TreeState>>
    where
        T: AsRef<Path>,
    {
        Ok(FileManager {
            state: TreeState::default(),
        })
    }
}

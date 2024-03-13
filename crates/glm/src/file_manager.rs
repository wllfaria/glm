use path_absolutize::*;
use std::path::{Path, PathBuf};

use crate::fs_ops::FsOps;
use crate::list_state::ListState;

/// FileType of a given item, in which can be `Directory | File | Symlink`
#[derive(Debug, Clone, PartialEq)]
pub enum FileType {
    Directory,
    File,
    Symlink,
}

/// `Item` is the representation of any contents of the filesystem.
#[derive(Debug, Clone)]
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
pub struct FileManager {
    state: ListState,
    show_hidden: bool,
}

impl FsOps<ListState> for FileManager {
    fn change_dir<P>(&mut self, path: P) -> anyhow::Result<&ListState>
    where
        P: AsRef<Path>,
    {
        let mut items = vec![];
        let path = path.as_ref().absolutize().unwrap();
        for entry in std::fs::read_dir(path.clone())? {
            let entry = entry?;
            let is_hidden = self.is_hidden(entry.path())?;

            if let (true, false) = (is_hidden, self.show_hidden) {
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

impl FileManager {
    pub fn new<T>(path: T) -> anyhow::Result<FileManager>
    where
        T: AsRef<Path>,
    {
        let mut fm = FileManager {
            state: ListState::default(),
            show_hidden: false,
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

    pub fn toggle_hidden(&mut self) -> anyhow::Result<&ListState> {
        self.show_hidden = !self.show_hidden;
        self.change_dir(self.state.current_dir.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::{tempdir, TempDir};

    fn setup_tempdir() -> TempDir {
        let dir = tempdir().expect("failed to create tempdir");

        for i in 0..10 {
            let file_name = i.to_string() + "tempfile.txt";
            let file_path = dir.path().join(file_name);
            let mut file = File::create(&file_path).expect("failed to create file");
            writeln!(file, "Hello, World!").expect("failed to write to file");
        }

        for i in 10..20 {
            let file_name = ".tempfile".to_owned() + &i.to_string() + ".txt";
            let file_path = dir.path().join(file_name);
            let mut file = File::create(&file_path).expect("failed to create file");
            writeln!(file, "Hello, World!").expect("failed to write to file");
        }
        dir
    }

    fn make_sut() -> (TempDir, FileManager) {
        let dir = setup_tempdir();
        let path = dir.path().to_path_buf();
        (
            dir,
            FileManager::new(path).expect("failed to create file manager"),
        )
    }

    #[test]
    fn test_new_file_manager() {
        let (dir, sut) = make_sut();

        let state = sut.get_state();

        assert_eq!(state.current_dir, dir.path());
        assert_eq!(state.items.len(), 10);
    }

    #[cfg(unix)]
    #[test]
    fn test_is_hidden() {
        let (_, sut) = make_sut();
        let state = sut.get_state();
        let not_hidden = &state.items[0].file_path;
        let hidden = "/fake/hidden/.path";

        let expect_true = sut.is_hidden(hidden).expect("failed to check filename");
        let expect_false = sut.is_hidden(not_hidden).expect("failed to check filename");

        assert!(expect_true);
        assert!(!expect_false);
    }

    #[test]
    fn test_is_file() {
        let (dir, sut) = make_sut();

        let state = sut.get_state();
        let expect_true = sut
            .is_file(&state.items[0].file_path)
            .expect("failed to check filename");
        let expect_false = sut.is_file(dir.path()).expect("failed to check tempdir");

        assert!(expect_true);
        assert!(!expect_false);
    }
}

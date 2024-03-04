use std::path::{Path, PathBuf};

use crate::item::{FileType, Item};

#[derive(Debug)]
pub struct FileManager {
    pub items: Vec<Item>,
    pub current_dir: PathBuf,
}

impl FileManager {
    pub fn new<T>(path: T) -> anyhow::Result<Self>
    where
        T: AsRef<Path>,
    {
        let mut file_manager = Self {
            items: vec![],
            current_dir: path.as_ref().to_path_buf(),
        };

        file_manager.populate()?;

        Ok(file_manager)
    }

    fn populate(&mut self) -> anyhow::Result<()> {
        let items = std::fs::read_dir(&self.current_dir)?;
        for item in items {
            let item = item?;
            let metadata = item.metadata()?;
            let file_size = metadata.len();
            let file_path = item.path();
            let file_name = file_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            let is_hidden = matches!(file_name.chars().next(), Some('.'));
            let file_type = match file_path.is_dir() {
                true => FileType::Directory,
                false => FileType::File,
            };
            let file_ext = match file_path.is_dir() {
                true => None,
                false => file_path
                    .extension()
                    .map(|ext| ext.to_string_lossy().to_string()),
            };
            let item = Item {
                file_name,
                file_path,
                file_size,
                is_hidden,
                file_type,
                file_ext,
                is_dirty: false,
            };
            self.items.push(item);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::{tempdir, TempDir};

    fn make_test_dir() -> anyhow::Result<(TempDir, PathBuf)> {
        let temp_dir = tempdir()?;
        let nested_dir = temp_dir.path().join("nested");
        fs::create_dir_all(&nested_dir)?;

        let temp_file = temp_dir.path().join("root_file.txt");
        let mut temp_file = File::create(temp_file)?;
        writeln!(temp_file, "Hello, World!")?;

        let nested_file = nested_dir.join("nested_file.txt");
        let mut nested_file = File::create(nested_file)?;
        writeln!(nested_file, "Hello, Nested!")?;
        Ok((temp_dir, nested_dir))
    }

    #[test]
    fn test_initalization() -> anyhow::Result<()> {
        let (dir, _) = make_test_dir()?;
        let sut = FileManager::new(dir.path())?;

        assert_eq!(sut.items.len(), 2);
        assert_eq!(sut.items[0].file_name, "nested");
        assert_eq!(sut.items[1].file_name, "root_file.txt");
        assert_eq!(sut.items[1].file_ext, Some("txt".into()));

        Ok(())
    }

    #[test]
    fn test_empty_dir() -> anyhow::Result<()> {
        let dir = tempdir()?;
        let sut = FileManager::new(dir.path())?;

        assert!(sut.items.is_empty());
        Ok(())
    }

    #[test]
    fn test_hidden_file() -> anyhow::Result<()> {
        let dir = tempdir()?;
        let file = dir.path().join(".hidden_file");
        File::create(file)?;

        let sut = FileManager::new(dir.path())?;

        assert!(sut.items[0].is_hidden);
        Ok(())
    }
}

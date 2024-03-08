use std::path::Path;

use crate::file_manager::FileType;

/// A common interface for base file system operations.
///
/// FsOps serves as a common way to perform file system operations that are
/// required by every variation of file managers, also requiring each
/// variation to implement possible specific behavior.
///
/// Some functions here are platform specific, make sure to check the function
/// for your operating system of interest.
pub trait FsOps<S> {
    /// Every file manager is required to implement this function. But every
    /// file manager can behave different. Check specific implementation for
    /// the file manager you need.
    fn change_dir<P>(&mut self, path: P) -> anyhow::Result<&S>
    where
        P: AsRef<Path>;

    /// Checks if a given path refers to a "hidden" file or directory on Unix-like systems.
    ///
    /// In Unix-like systems, a file is considered hidden if its name starts with
    /// a dot (`.`). This function examines the final component of the provided path
    /// to determine if it adheres to this convention.
    ///
    /// Returns `true` if the path refers to a hidden file or directory. Or false if the
    /// path does not follow the convention, or if any of the path processing fails.
    /// (e.g., if the file name cannot be converted to a string).
    #[cfg(unix)]
    fn is_hidden<P>(&self, path: P) -> anyhow::Result<bool>
    where
        P: AsRef<Path>,
    {
        Ok(path
            .as_ref()
            .file_name()
            .and_then(|name| name.to_str())
            .map(|name| name.starts_with('.'))
            .unwrap_or(false))
    }

    #[cfg(windows)]
    fn is_hidden<P>(&self, path: P) -> bool
    where
        P: AsRef<Path>,
    {
        todo!()
    }

    /// Testes whether a given path is a symbolic link.
    fn is_symlink<P>(&self, path: P) -> anyhow::Result<bool>
    where
        P: AsRef<Path>,
    {
        let metadata = std::fs::symlink_metadata(path)?;
        let file_type = metadata.file_type();
        Ok(file_type.is_symlink())
    }

    /// Testes whether a given path represents a directory.
    fn is_dir<P>(&self, path: P) -> anyhow::Result<bool>
    where
        P: AsRef<Path>,
    {
        let metadata = std::fs::metadata(path)?;
        let file_type = metadata.file_type();
        Ok(file_type.is_dir())
    }

    /// Testes whether a given path represents a regular file
    fn is_file<P>(&self, path: P) -> anyhow::Result<bool>
    where
        P: AsRef<Path>,
    {
        let metadata = std::fs::metadata(path)?;
        let file_type = metadata.file_type();
        Ok(file_type.is_file())
    }

    /// Returns the `FileType` of a given path. by checking if the path is
    /// a symbolic link, directory or a regular file.
    ///
    /// # Returns
    /// * `FileType::Symlink` if the call to `self.is_symlink(path)` returns true
    /// * `FileType::Directory` if the call to `self.is_dir(path)` returns true
    /// * `FileType::File` if the call to `self.is_file(path)` returns true
    fn get_file_type<P>(&self, path: P) -> anyhow::Result<FileType>
    where
        P: AsRef<Path>,
    {
        let is_symlink = self.is_symlink(path.as_ref())?;
        let is_dir = self.is_dir(path.as_ref())?;
        if is_symlink {
            Ok(FileType::Symlink)
        } else if is_dir {
            Ok(FileType::Directory)
        } else {
            Ok(FileType::File)
        }
    }

    /// Returns the extension of a given path either as an `String` for regular
    /// files, or `None` for directories.
    fn get_file_extension<P>(&self, path: P) -> Option<String>
    where
        P: AsRef<Path>,
    {
        path.as_ref()
            .extension()
            .map(|ext| ext.to_string_lossy().to_string())
    }
}


use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub enum FileError {
    NotFound,
    Error(std::io::Error),
}

pub fn copy_recursive(
    source: &Path,
    destination: &Path,
    exclude: &[PathBuf],
) -> Result<(), FileError> {
    for entry in fs::read_dir(source).map_err(FileError::Error)? {
        let entry = entry.map_err(FileError::Error)?;
        let path = entry.path();
        let filename = path
            .file_name()
            .ok_or(FileError::Error(std::io::ErrorKind::InvalidInput.into()))?;
        let destination = destination.join(filename);

        // if glob matches filename, e.g. *.ts* should remove any file extension that starts with .ts
        if exclude.iter().any(|p| p.is_dir()) {
            // if filename matches directory
            if exclude
                .iter()
                .any(|p| p.is_dir() && p.file_name() == Some(filename))
            {
                println!(
                    "\x1b[1;31m 2 Skipping {:?} because it's in exclude\x1b[0m",
                    &path
                );
                continue;
            }
        }

        // if filename is *.ts (or any other glob pattern)
        if exclude.iter().any(|p| p.is_file()) {
            // if filename matches file
            if exclude
                .iter()
                .any(|p| p.is_file() && p.file_name() == Some(filename))
            {
                println!(
                    "\x1b[1;31m 3 Skipping {:?} because it's in exclude\x1b[0m",
                    &path
                );
                continue;
            }
        }

        if path.is_dir() {
            fs::create_dir_all(&destination).map_err(FileError::Error)?;
            copy_recursive(&path, &destination, exclude)?;
        } else {
            fs::copy(&path, &destination).map_err(FileError::Error)?;
        }
    }
    Ok(())
}

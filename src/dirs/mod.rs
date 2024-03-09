use std::env::var_os;
use std::path::PathBuf;

use crate::utils::config::ConfigError;

pub fn config_dir() -> Result<PathBuf, ConfigError> {
    let path = home_dir()?;

    #[cfg(target_os = "macos")]
    return Ok(path.join("Library/Application Support"));

    #[cfg(target_os = "windows")]
    return Ok(path.join(r"AppData\Roaming"));

    #[cfg(target_os = "linux")]
    return Ok(path.join(".config"));
}

fn home_dir() -> Result<PathBuf, ConfigError> {
    let home = var_os("HOME").ok_or(ConfigError::DirNotFound)?;
    let path_buf = PathBuf::from(home);
    if path_buf.is_dir() {
        return Ok(path_buf);
    }

    let userprofile = var_os("USERPROFILE").ok_or(ConfigError::DirNotFound)?;
    let path_buf = PathBuf::from(userprofile);
    if path_buf.is_dir() {
        return Ok(path_buf);
    }

    Err(ConfigError::DirNotFound)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_dir_mac() {
        let path = config_dir().expect(
            "Failed to get config directory. Make sure you have a HOME environment variable.",
        );
        assert!(path
            .to_string_lossy()
            .contains("/Library/Application Support"));
    }
}

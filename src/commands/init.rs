use std::{fs, io::ErrorKind};

use crate::utils::get_config;

pub fn run(_namespace: Option<String>) {
    let config = get_config();

    let config_dirname = dirs::data_dir()
        .expect("Data directory not found")
        .join("cloup");

    if let Err(e) = fs::create_dir(&config_dirname) {
        #[allow(clippy::single_match)]
        match e.kind() {
            ErrorKind::PermissionDenied => {
                eprintln!("Permission denied when creating config directory")
            }
            _ => (),
        }
    }

    // TODO: Improve config file, use config-managable crate for this

    fs::write(
        config_dirname.join(".config"),
        format!("template_dir={:?}", config.current_dir),
    )
    .expect("An error occurred when writing config file");

    println!("📚 Successfully made this the template directory for cloups");
}

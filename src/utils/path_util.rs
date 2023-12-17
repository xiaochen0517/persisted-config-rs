use std::io::Error;
use std::path::PathBuf;

pub(crate) fn get_config_path(config_path: &str) -> Result<PathBuf, Error> {
    let exe_full_path = std::env::current_exe()?;
    let mut exe_dir_path = exe_full_path.parent().unwrap().to_path_buf();
    exe_dir_path.push(config_path);
    // create the config directory if it doesn't exist
    if !exe_dir_path.exists() {
        std::fs::create_dir_all(&exe_dir_path)?;
    }
    Ok(exe_dir_path)
}
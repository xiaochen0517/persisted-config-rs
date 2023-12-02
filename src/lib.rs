use std::io::{Read, Write};

pub(crate) mod utils;

/// Save config file.
///
/// # Arguments
///
/// * `file_name` - The name of the config file.
/// * `data` - The data to be saved.
///
/// # Examples
///
/// ```rust
/// use persisted_config_rs::save_config;
///
/// let file_name = "test.txt";
/// let data = "Hello, world!";
/// save_config(file_name, data)?; // Ok(())
/// ```
pub fn save_config(file_name: &str, data: &str) -> Result<(), std::io::Error> {
    let mut path = utils::path_util::get_config_path()?;
    path.push(file_name);
    let mut file = utils::file_util::get_file_by_path(path)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

/// Get config file.
///
/// # Arguments
///
/// * `file_name` - The name of the config file.
///
/// # Examples
///
/// ```rust
/// use persisted_config_rs::get_config;
///
/// let file_name = "test.txt";
/// let data = get_config(file_name);
/// ```
pub fn get_config(file_name: &str) -> Result<String, std::io::Error> {
    let mut path = utils::path_util::get_config_path()?;
    path.push(file_name);
    let mut file = utils::file_util::get_file_by_path(path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    Ok(data)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_save_config() {
        let file_name = "test.txt";
        let data = "Hello, world!";
        let result = super::save_config(file_name, data);
        println!("result: {:?}", result);
        assert!(result.is_ok());
    }
}

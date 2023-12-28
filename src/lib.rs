use std::io::{Read, Write};

pub(crate) mod utils;

/// PersistedConfig is a struct that represents a persisted config.
pub struct PersistedConfig {

    /// The path to the config directory.
    pub config_path: String,
}

impl PersistedConfig {

    /// Create a new PersistedConfig.
    ///
    /// # Arguments
    ///
    /// * `config_path` - The path to the config directory.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use persisted_config_rs::PersistedConfig;
    ///
    /// let config = PersistedConfig::new("config");
    /// ```
    pub fn new(config_path: &str) -> PersistedConfig {
        PersistedConfig {
            config_path: config_path.to_string(),
        }
    }

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
    /// use persisted_config_rs::PersistedConfig;
    ///
    /// fn main() -> Result<(), std::io::Error> {
    ///     let file_name = "test.txt";
    ///     let data = "Hello, world!";
    ///     PersistedConfig::new("config").save_config(file_name, data)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn save_config(&self, file_name: &str, data: &str) -> Result<(), std::io::Error> {
        let mut path = utils::path_util::get_config_path(&self.config_path)?;
        path.push(file_name);
        let mut file = utils::file_util::get_file_by_path(path, true)?;
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
    /// use persisted_config_rs::PersistedConfig;
    ///
    /// let file_name = "test.txt";
    /// let data = PersistedConfig::new("config").get_config(file_name);
    /// ```
    pub fn get_config(&self, file_name: &str) -> Result<String, std::io::Error> {
        let mut path = utils::path_util::get_config_path(&self.config_path)?;
        path.push(file_name);
        let mut file = utils::file_util::get_file_by_path(path, false)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_save_and_get_config() {
        let file_name = "test.txt";
        let data = "Hello, world!";
        let config_path = "config";
        let config = super::PersistedConfig::new(config_path);
        let result = config.save_config(file_name, data);
        println!("save result: {:?}", result);
        assert!(result.is_ok());
        let result = config.get_config(file_name);
        println!("get result: {:?}", result);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), data);
    }
}

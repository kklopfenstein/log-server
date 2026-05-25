use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct LogConfig {
    #[serde(flatten)]
    pub files: HashMap<String, String>,
}

impl LogConfig {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: LogConfig = serde_yaml::from_str(&content)?;
        
        // Validate that all configured files exist
        for (name, file_path) in &config.files {
            if !Path::new(file_path).exists() {
                return Err(format!(
                    "Configured log file '{}' for '{}' does not exist",
                    file_path, name
                ).into());
            }
        }
        
        Ok(config)
    }
    
    pub fn get_path(&self, name: &str) -> Option<&String> {
        self.files.get(name)
    }
}
use std::collections::HashMap;

pub type Files = HashMap<String, String>;

#[derive(Debug, Default, Clone)]
pub struct LogConfig {
    pub files: Option<Files>,
}

impl LogConfig {
    pub fn load(config_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_content = std::fs::read_to_string(config_path)?;
        let root: serde_yaml::Value = serde_yaml::from_str(&config_content)?;
        
        if let Some(v) = root.get("logs") {
            if let Some(mapping) = v.as_mapping() {
                // Only create files if mapping has at least one entry
                if mapping.len() > 0 {
                    let files = mapping
                        .iter()
                        .map(|(k, v)| {
                            let k = k.as_str().unwrap_or("").to_string();
                            let v = v.as_str().unwrap_or_default().to_string();
                            (k, v)
                        })
                        .collect();
                    return Ok(LogConfig { files: Some(files) });
                }
            }
        }
        
        Ok(LogConfig::default())
    }

    pub fn get(&self, name: &str) -> Option<&String> {
        self.files.as_ref().and_then(|files| files.get(name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{io::Write, path::PathBuf};

    static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

    fn create_temp_config(content: &str) -> PathBuf {
        let counter = COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let temp_dir = std::env::temp_dir();
        let config_path = temp_dir.join(format!(
            "test_log_config_{}.yaml",
            counter
        ));
        let mut file = std::fs::File::create(&config_path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        config_path
    }

    #[test]
    fn test_load_valid_config() {
        let config_yaml = r#"
logs:
    foo.log: path/to/foo.log
    bar.log: path/to/bar.log
"#;
        let config_path = create_temp_config(config_yaml);

        let result = LogConfig::load(&config_path.to_string_lossy());
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(config.get("foo.log"), Some(&"path/to/foo.log".to_string()));
        assert_eq!(config.get("bar.log"), Some(&"path/to/bar.log".to_string()));
    }

    #[test]
    fn test_load_empty_logs_section() {
        let config_yaml = r#"logs: {}"#;
        let config_path = create_temp_config(config_yaml);

        let result = LogConfig::load(&config_path.to_string_lossy());
        assert!(result.is_ok());

        let config = result.unwrap();
        assert!(config.files.is_none());
    }

    #[test]
    fn test_load_no_logs_section() {
        let config_yaml = r#"name: my-app"#;
        let config_path = create_temp_config(config_yaml);

        let result = LogConfig::load(&config_path.to_string_lossy());
        assert!(result.is_ok());

        let config = result.unwrap();
        assert!(config.files.is_none());
    }

    #[test]
    fn test_load_nonexistent_config() {
        let config_path = std::env::temp_dir().join("nonexistent_test.yaml");
        let config_path_str = config_path.to_string_lossy().to_string();

        let result = LogConfig::load(&config_path_str);
        assert!(result.is_err());
    }
}

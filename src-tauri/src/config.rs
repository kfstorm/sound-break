use crate::meeting_detector::MeetingConfig;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub meeting_config: MeetingConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            meeting_config: MeetingConfig::default(),
        }
    }
}

pub struct ConfigManager;

impl ConfigManager {
    fn get_config_path() -> Result<PathBuf, String> {
        let config_dir = dirs::config_dir()
            .ok_or("Could not find config directory")?;
        
        let app_config_dir = config_dir.join("com.kfstorm.sound-break");
        
        // Create the directory if it doesn't exist
        std::fs::create_dir_all(&app_config_dir)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;
        
        Ok(app_config_dir.join("config.json"))
    }

    pub fn load_config() -> AppConfig {
        match Self::get_config_path() {
            Ok(config_path) => {
                if config_path.exists() {
                    match std::fs::read_to_string(&config_path) {
                        Ok(content) => {
                            match serde_json::from_str::<AppConfig>(&content) {
                                Ok(config) => {
                                    println!("SoundBreak: Loaded configuration from {:?}", config_path);
                                    return config;
                                }
                                Err(e) => {
                                    eprintln!("SoundBreak: Failed to parse config file: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("SoundBreak: Failed to read config file: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("SoundBreak: Failed to get config path: {}", e);
            }
        }
        
        println!("SoundBreak: Using default configuration");
        AppConfig::default()
    }

    pub fn save_config(config: &AppConfig) -> Result<(), String> {
        let config_path = Self::get_config_path()?;
        
        let json_content = serde_json::to_string_pretty(config)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;
        
        std::fs::write(&config_path, json_content)
            .map_err(|e| format!("Failed to write config file: {}", e))?;
        
        println!("SoundBreak: Saved configuration to {:?}", config_path);
        Ok(())
    }
}
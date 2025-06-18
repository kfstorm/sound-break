use serde::{Deserialize, Serialize};
use sysinfo::System;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeetingApp {
    pub name: String,
    pub process_name: String,
    pub is_running: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeetingStatus {
    pub in_meeting: bool,
    pub active_apps: Vec<MeetingApp>,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeetingConfig {
    pub process_names: Vec<String>,
}

impl Default for MeetingConfig {
    fn default() -> Self {
        Self {
            process_names: vec![
                "Lark Helper (Iron)".to_string(),
            ],
        }
    }
}

pub struct MeetingDetector {
    config: MeetingConfig,
    system: System,
}

impl MeetingDetector {
    pub fn new() -> Self {
        Self {
            config: MeetingConfig::default(),
            system: System::new_all(),
        }
    }

    pub fn update_config(&mut self, config: MeetingConfig) {
        self.config = config;
    }

    pub fn get_config(&self) -> &MeetingConfig {
        &self.config
    }

    pub fn detect_meetings(&mut self) -> MeetingStatus {
        // Refresh the system information to get current processes
        self.system.refresh_all();

        let mut active_apps = Vec::new();
        let mut in_meeting = false;

        for process_name in &self.config.process_names {
            if self.is_process_running(process_name) {
                active_apps.push(MeetingApp {
                    name: process_name.clone(),
                    process_name: process_name.clone(),
                    is_running: true,
                });
                in_meeting = true;
            }
        }

        MeetingStatus {
            in_meeting,
            active_apps,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    fn is_process_running(&self, process_name: &str) -> bool {
        // Check all running processes for exact name match
        for (_, process) in self.system.processes() {
            if process.name().to_string_lossy() == process_name {
                return true;
            }
        }
        false
    }
}

impl Default for MeetingDetector {
    fn default() -> Self {
        Self::new()
    }
}

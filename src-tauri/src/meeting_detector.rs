//! Meeting Detector Module
//!
//! This module provides functionality to detect when meeting applications are running
//! and determine if the user is currently in a meeting.
//!
//! ## Process Detection Behavior
//!
//! **EXACT MATCHING ONLY**: This module uses exact string matching for process names.
//! - Process names must match exactly as they appear in the system process list
//! - No partial matching, substring matching, or fuzzy matching is performed
//! - Case-sensitive matching is used
//! - Uses `pgrep` command with regex anchors (^pattern$) for reliable detection
//!
//! ## Implementation Details
//!
//! The detection is implemented using the `pgrep` system command which:
//! - Provides real-time process status without caching issues
//! - Uses regex pattern matching with start (^) and end ($) anchors
//! - Automatically handles process name escaping for special characters
//! - Returns immediately accurate results when processes start/stop
//!
//! ## Configuration
//!
//! Meeting applications are detected by their process names, which must be configured
//! exactly as they appear when running. Common examples:
//! - "Lark Helper (Iron)" - Feishu/Lark meeting helper process
//! - "zoom.us" - Zoom meeting client
//! - "Microsoft Teams" - Teams application
//!
//! ## Usage Notes
//!
//! To find the exact process name for a meeting application:
//! 1. Start the meeting application
//! 2. Run `pgrep -l <partial_name>` in terminal to see running processes
//! 3. Use the exact process name shown in the output
//!
//! ## Process Name Examples
//!
//! Different meeting apps may have different process names:
//! - Feishu: "Lark Helper (Iron)", "Feishu", "Lark"
//! - Zoom: "zoom.us", "Zoom"
//! - Teams: "Microsoft Teams", "Teams"
//! - WebEx: "Cisco Webex Meetings", "ptoneclk"
//!
//! Always verify the exact process name using `pgrep -l` when the app is running.

use serde::{Deserialize, Serialize};

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
                "TencentMeeting".to_string(),
            ],
        }
    }
}

pub struct MeetingDetector {
    config: MeetingConfig,
}

impl MeetingDetector {
    pub fn new() -> Self {
        Self {
            config: MeetingConfig::default(),
        }
    }

    pub fn update_config(&mut self, config: MeetingConfig) {
        self.config = config;
    }

    pub fn get_config(&self) -> &MeetingConfig {
        &self.config
    }

    pub fn detect_meetings(&mut self) -> MeetingStatus {
        let mut active_apps = Vec::new();
        let mut in_meeting = false;

        for process_name in &self.config.process_names {
            let is_running = self.is_process_running(process_name);

            // Always add the app to the list with its current status
            active_apps.push(MeetingApp {
                name: process_name.clone(),
                process_name: process_name.clone(),
                is_running,
            });

            if is_running {
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
        use std::process::Command;

        // Use pgrep for exact process name matching
        // ^pattern$ ensures exact match, no partial matching
        let pattern = format!("^{}$", regex::escape(process_name));

        let output = Command::new("pgrep")
            .arg(&pattern)
            .output();

        match output {
            Ok(result) => {
                // Process exists if pgrep returns success and has output
                result.status.success() && !result.stdout.is_empty()
            }
            Err(_) => {
                // If pgrep command fails, assume process doesn't exist
                false
            }
        }
    }
}

impl Default for MeetingDetector {
    fn default() -> Self {
        Self::new()
    }
}

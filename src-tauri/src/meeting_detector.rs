use serde::{Deserialize, Serialize};
use std::process::Command;

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

pub struct MeetingDetector {
    meeting_apps: Vec<MeetingApp>,
}

impl MeetingDetector {
    pub fn new() -> Self {
        let meeting_apps = vec![
            MeetingApp {
                name: "Feishu Meeting".to_string(),
                process_name: "feishu".to_string(),
                is_running: false,
            },
            MeetingApp {
                name: "Lark Meeting".to_string(),
                process_name: "lark".to_string(),
                is_running: false,
            },
            MeetingApp {
                name: "Zoom".to_string(),
                process_name: "zoom".to_string(),
                is_running: false,
            },
            MeetingApp {
                name: "Microsoft Teams".to_string(),
                process_name: "teams".to_string(),
                is_running: false,
            },
        ];

        Self {
            meeting_apps,
        }
    }

    pub fn detect_meetings(&mut self) -> MeetingStatus {
        let mut active_apps = Vec::new();
        let mut in_meeting = false;

        // Check for Feishu/Lark meeting windows
        if self.is_feishu_in_meeting() {
            active_apps.push(MeetingApp {
                name: "Feishu Meeting".to_string(),
                process_name: "feishu".to_string(),
                is_running: true,
            });
            in_meeting = true;
        }

        // Check for Zoom meetings
        if self.is_zoom_in_meeting() {
            active_apps.push(MeetingApp {
                name: "Zoom".to_string(),
                process_name: "zoom".to_string(),
                is_running: true,
            });
            in_meeting = true;
        }

        // Check for Teams meetings
        if self.is_teams_in_meeting() {
            active_apps.push(MeetingApp {
                name: "Microsoft Teams".to_string(),
                process_name: "teams".to_string(),
                is_running: true,
            });
            in_meeting = true;
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

    fn is_feishu_in_meeting(&self) -> bool {
        // Method 1: Check for Iron process (meeting engine)
        if self.is_iron_process_active() {
            // Method 2: Check for network connections to meeting servers
            if self.has_meeting_network_activity() {
                return true;
            }

            // Method 3: Check for audio/video device usage
            if self.is_using_media_devices() {
                return true;
            }
        }

        // Method 4: Check for meeting-specific processes or arguments
        self.has_meeting_process_indicators()
    }

    fn is_iron_process_active(&self) -> bool {
        // Check if Lark Helper (Iron) process is running
        match Command::new("pgrep").arg("-f").arg("Lark Helper \\(Iron\\)").output() {
            Ok(output) => !output.stdout.is_empty(),
            Err(_) => false,
        }
    }

    fn has_meeting_network_activity(&self) -> bool {
        // Check for network connections to Feishu/Lark meeting servers
        match Command::new("netstat")
            .arg("-an")
            .arg("-p")
            .arg("tcp")
            .output() {
            Ok(output) => {
                let netstat_output = String::from_utf8_lossy(&output.stdout);
                // Look for connections to Feishu/Lark meeting server domains
                netstat_output.contains("feishu.cn") ||
                netstat_output.contains("larksuite.com") ||
                netstat_output.contains("bytedance.net") ||
                // Common WebRTC ports used by meeting applications
                (netstat_output.contains(":3478") || // STUN
                 netstat_output.contains(":5349") || // TURNS
                 netstat_output.contains(":19302"))  // Google STUN
            }
            Err(_) => false,
        }
    }

    fn is_using_media_devices(&self) -> bool {
        // Check if any Lark process is using audio/video devices
        // Use lsof to check for device access
        match Command::new("lsof")
            .arg("-c")
            .arg("Lark")
            .arg("-c")
            .arg("Feishu")
            .output() {
            Ok(output) => {
                let lsof_output = String::from_utf8_lossy(&output.stdout);
                // Look for audio/video device access
                lsof_output.contains("/dev/") && (
                    lsof_output.contains("audio") ||
                    lsof_output.contains("video") ||
                    lsof_output.contains("camera") ||
                    lsof_output.contains("microphone") ||
                    lsof_output.contains("CoreAudio") ||
                    lsof_output.contains("AVFoundation")
                )
            }
            Err(_) => false,
        }
    }

    fn has_meeting_process_indicators(&self) -> bool {
        // Check process arguments and environment for meeting indicators
        match Command::new("ps")
            .arg("-eo")
            .arg("pid,comm,args")
            .arg("-c")
            .arg("Lark")
            .arg("-c")
            .arg("Feishu")
            .output() {
            Ok(output) => {
                let ps_output = String::from_utf8_lossy(&output.stdout);
                // Look for meeting-related arguments or keywords
                ps_output.contains("meeting") ||
                ps_output.contains("conference") ||
                ps_output.contains("webrtc") ||
                ps_output.contains("--meeting") ||
                ps_output.contains("--conference-mode") ||
                ps_output.contains("iron") // Iron engine typically handles meetings
            }
            Err(_) => false,
        }
    }

    fn is_zoom_in_meeting(&self) -> bool {
        // Check for Zoom processes and network activity
        if self.is_zoom_process_active() {
            // Check for meeting-related network activity
            self.has_zoom_meeting_network_activity() || self.is_zoom_using_media()
        } else {
            false
        }
    }

    fn is_zoom_process_active(&self) -> bool {
        match Command::new("pgrep").arg("-i").arg("zoom").output() {
            Ok(output) => !output.stdout.is_empty(),
            Err(_) => false,
        }
    }

    fn has_zoom_meeting_network_activity(&self) -> bool {
        match Command::new("netstat").arg("-an").arg("-p").arg("tcp").output() {
            Ok(output) => {
                let netstat_output = String::from_utf8_lossy(&output.stdout);
                netstat_output.contains("zoom.us") ||
                netstat_output.contains("zoomgov.com") ||
                // Zoom uses various ports for meetings
                netstat_output.contains(":8801") ||
                netstat_output.contains(":8802")
            }
            Err(_) => false,
        }
    }

    fn is_zoom_using_media(&self) -> bool {
        match Command::new("lsof").arg("-c").arg("zoom").output() {
            Ok(output) => {
                let lsof_output = String::from_utf8_lossy(&output.stdout);
                lsof_output.contains("CoreAudio") ||
                lsof_output.contains("AVFoundation") ||
                lsof_output.contains("/dev/")
            }
            Err(_) => false,
        }
    }

    fn is_teams_in_meeting(&self) -> bool {
        // Check for Teams processes and network activity
        if self.is_teams_process_active() {
            self.has_teams_meeting_network_activity() || self.is_teams_using_media()
        } else {
            false
        }
    }

    fn is_teams_process_active(&self) -> bool {
        match Command::new("pgrep").arg("-i").arg("teams").output() {
            Ok(output) => !output.stdout.is_empty(),
            Err(_) => false,
        }
    }

    fn has_teams_meeting_network_activity(&self) -> bool {
        match Command::new("netstat").arg("-an").arg("-p").arg("tcp").output() {
            Ok(output) => {
                let netstat_output = String::from_utf8_lossy(&output.stdout);
                netstat_output.contains("teams.microsoft.com") ||
                netstat_output.contains("teams.live.com") ||
                netstat_output.contains("skype.com")
            }
            Err(_) => false,
        }
    }

    fn is_teams_using_media(&self) -> bool {
        match Command::new("lsof").arg("-c").arg("teams").output() {
            Ok(output) => {
                let lsof_output = String::from_utf8_lossy(&output.stdout);
                lsof_output.contains("CoreAudio") ||
                lsof_output.contains("AVFoundation") ||
                lsof_output.contains("/dev/")
            }
            Err(_) => false,
        }
    }

    pub fn is_feishu_meeting_active(&mut self) -> bool {
        self.is_feishu_in_meeting()
    }
}

impl Default for MeetingDetector {
    fn default() -> Self {
        Self::new()
    }
}

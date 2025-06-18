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
        // Use AppleScript to check for Feishu meeting windows or audio/camera usage
        let script = r#"
            tell application "System Events"
                try
                    -- Check if Feishu/Lark is using camera or microphone
                    set feishuInMeeting to false
                    
                    -- Method 1: Check for meeting-specific windows
                    repeat with proc in (every process whose name contains "Feishu" or name contains "Lark")
                        repeat with win in (every window of proc)
                            set winTitle to title of win
                            -- Look for meeting window indicators
                            if winTitle contains "会议" or winTitle contains "Meeting" or winTitle contains "通话" or winTitle contains "Call" then
                                set feishuInMeeting to true
                                exit repeat
                            end if
                        end repeat
                        if feishuInMeeting then exit repeat
                    end repeat
                    
                    return feishuInMeeting as string
                on error
                    return "false"
                end try
            end tell
        "#;

        match Command::new("osascript").arg("-e").arg(script).output() {
            Ok(output) => {
                let result = String::from_utf8_lossy(&output.stdout).trim();
                result == "true"
            }
            Err(_) => false,
        }
    }

    fn is_zoom_in_meeting(&self) -> bool {
        let script = r#"
            tell application "System Events"
                try
                    set zoomInMeeting to false
                    repeat with proc in (every process whose name contains "zoom")
                        repeat with win in (every window of proc)
                            set winTitle to title of win
                            if winTitle contains "Zoom Meeting" or winTitle contains "Meeting" then
                                set zoomInMeeting to true
                                exit repeat
                            end if
                        end repeat
                        if zoomInMeeting then exit repeat
                    end repeat
                    return zoomInMeeting as string
                on error
                    return "false"
                end try
            end tell
        "#;

        match Command::new("osascript").arg("-e").arg(script).output() {
            Ok(output) => {
                let result = String::from_utf8_lossy(&output.stdout).trim();
                result == "true"
            }
            Err(_) => false,
        }
    }

    fn is_teams_in_meeting(&self) -> bool {
        let script = r#"
            tell application "System Events"
                try
                    set teamsInMeeting to false
                    repeat with proc in (every process whose name contains "Teams")
                        repeat with win in (every window of proc)
                            set winTitle to title of win
                            if winTitle contains "Meeting" or winTitle contains "Call" then
                                set teamsInMeeting to true
                                exit repeat
                            end if
                        end repeat
                        if teamsInMeeting then exit repeat
                    end repeat
                    return teamsInMeeting as string
                on error
                    return "false"
                end try
            end tell
        "#;

        match Command::new("osascript").arg("-e").arg(script).output() {
            Ok(output) => {
                let result = String::from_utf8_lossy(&output.stdout).trim();
                result == "true"
            }
            Err(_) => false,
        }
    }

    pub fn is_feishu_meeting_active(&mut self) -> bool {
        self.is_feishu_in_meeting()
    }
}

impl Default for MeetingDetector {

impl Default for MeetingDetector {
    fn default() -> Self {
        Self::new()
    }
}

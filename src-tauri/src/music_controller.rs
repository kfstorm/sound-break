use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicStatus {
    pub is_playing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MusicAction {
    Play,
    Pause,
}

pub struct MusicController;

impl MusicController {
    pub fn new() -> Self {
        Self
    }

    pub fn get_music_status(&self) -> MusicStatus {
        // Only check for system-wide audio activity
        let is_playing = self.is_system_audio_playing();

        MusicStatus {
            is_playing,
        }
    }


    fn is_system_audio_playing(&self) -> bool {
        // Method 1: Check for audio assertions (most reliable universal method)
        if self.has_audio_assertions() {
            return true;
        }

        // Method 2: Check for active audio processes
        self.has_active_audio_processes()
    }

    fn has_audio_assertions(&self) -> bool {
        // Check if coreaudiod has active audio assertions (prevents system sleep during audio playback)
        match Command::new("pmset").arg("-g").arg("assertions").output() {
            Ok(output) => {
                let assertions_output = String::from_utf8_lossy(&output.stdout);
                // Look for audio-related assertions that prevent sleep
                assertions_output.contains("coreaudiod") &&
                (assertions_output.contains("PreventUserIdleSystemSleep") ||
                 assertions_output.contains("audio-out") ||
                 assertions_output.contains("Audio"))
            }
            Err(_) => false,
        }
    }

    fn has_active_audio_processes(&self) -> bool {
        // Check for processes actively using audio resources
        match Command::new("lsof").arg("-n").arg("-P").output() {
            Ok(output) => {
                let lsof_output = String::from_utf8_lossy(&output.stdout);
                // Look for active audio usage patterns
                lsof_output.lines().filter(|line| {
                    line.contains("CoreAudio") ||
                    line.contains("AudioUnit") ||
                    line.contains("AVFoundation") ||
                    line.contains("audio-out")
                }).count() > 2 // More than just system processes
            }
            Err(_) => false,
        }
    }

    pub fn play_music(&self) -> Result<String, String> {
        self.send_media_key("play")
    }

    pub fn pause_music(&self) -> Result<String, String> {
        self.send_media_key("pause")
    }

    fn send_media_key(&self, action: &str) -> Result<String, String> {
        let key_code = match action {
            "play" => "16", // F7 - Play/Pause
            "pause" => "16", // F7 - Play/Pause (same key toggles)
            _ => return Err(format!("Unknown media action: {}", action)),
        };

        let script = format!(
            r#"
            try
                tell application "System Events"
                    key code {} using {{command down}}
                    return "Media key {} sent"
                end tell
            on error errMsg
                return "Error: " & errMsg
            end try
            "#,
            key_code, action
        );

        match Command::new("osascript").arg("-e").arg(&script).output() {
            Ok(output) => {
                let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if result.starts_with("Error:") {
                    Err(result)
                } else {
                    Ok(result)
                }
            }
            Err(e) => Err(format!("Failed to send media key: {}", e)),
        }
    }

    pub fn execute_action(&self, action: MusicAction) -> Result<String, String> {
        match action {
            MusicAction::Play => self.play_music(),
            MusicAction::Pause => self.pause_music(),
        }
    }
}

impl Default for MusicController {
    fn default() -> Self {
        Self::new()
    }
}

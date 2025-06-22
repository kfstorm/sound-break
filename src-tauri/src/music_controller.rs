use serde::{Deserialize, Serialize};
use std::process::Command;
use std::path::PathBuf;

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
        // Use nowplaying-cli for universal music detection
        let is_playing = self.is_music_playing_via_nowplaying();

        MusicStatus {
            is_playing,
        }
    }


    fn is_music_playing_via_nowplaying(&self) -> bool {
        // First try the MediaRemote framework approach for macOS 15.4+
        if let Ok(is_playing) = self.check_music_via_mediaremote() {
            return is_playing;
        }

        // Fallback to nowplaying-cli if MediaRemote fails
        let nowplaying_path = self.get_nowplaying_cli_path();

        match Command::new(&nowplaying_path).arg("get").arg("playbackRate").output() {
            Ok(output) => {
                let output_str = String::from_utf8_lossy(&output.stdout);
                let rate = output_str.trim();
                rate.parse::<f32>().unwrap_or(0.0) > 0.0
            }
            Err(_) => false, // nowplaying-cli not available
        }
    }

    fn check_music_via_mediaremote(&self) -> Result<bool, String> {
        // Use AppleScript with MediaRemote framework for macOS 15.4+ compatibility
        let script = r#"
            use framework "AppKit"
            try
                set MediaRemote to current application's NSBundle's bundleWithPath:"/System/Library/PrivateFrameworks/MediaRemote.framework/"
                MediaRemote's load()

                set MRNowPlayingRequest to current application's NSClassFromString("MRNowPlayingRequest")
                set infoDict to MRNowPlayingRequest's localNowPlayingItem()'s nowPlayingInfo()

                if infoDict is missing value then
                    return "false"
                end if

                set playbackRate to infoDict's valueForKey:"kMRMediaRemoteNowPlayingInfoPlaybackRate"
                if playbackRate is missing value then
                    return "false"
                end if

                set rateValue to playbackRate as real
                if rateValue > 0 then
                    return "true"
                else
                    return "false"
                end if
            on error
                return "false"
            end try
        "#;

        match Command::new("osascript").arg("-e").arg(script).output() {
            Ok(output) => {
                let result_str = String::from_utf8_lossy(&output.stdout);
                let result = result_str.trim();
                Ok(result == "true")
            }
            Err(e) => Err(format!("MediaRemote check failed: {}", e)),
        }
    }

    pub fn play_music(&self) -> Result<String, String> {
        self.send_nowplaying_command("play")
    }

    pub fn pause_music(&self) -> Result<String, String> {
        self.send_nowplaying_command("pause")
    }

    fn send_nowplaying_command(&self, action: &str) -> Result<String, String> {
        // First try MediaRemote framework approach for macOS 15.4+
        if let Ok(result) = self.send_mediaremote_command(action) {
            return Ok(result);
        }

        // Fallback to nowplaying-cli
        let nowplaying_path = self.get_nowplaying_cli_path();

        match Command::new(&nowplaying_path).arg("set").arg(action).output() {
            Ok(output) => {
                if output.status.success() {
                    let result_str = String::from_utf8_lossy(&output.stdout);
                    let result = result_str.trim();
                    if result.is_empty() {
                        Ok(format!("Music {} command sent successfully via nowplaying-cli", action))
                    } else {
                        Ok(result.to_string())
                    }
                } else {
                    let error_str = String::from_utf8_lossy(&output.stderr);
                    let error = error_str.trim();
                    if error.is_empty() {
                        Err(format!("Failed to {} music: command failed", action))
                    } else {
                        Err(format!("Failed to {} music: {}", action, error))
                    }
                }
            }
            Err(e) => Err(format!("Failed to execute nowplaying-cli {}: {}", action, e)),
        }
    }

    fn send_mediaremote_command(&self, action: &str) -> Result<String, String> {
        // Map action to MediaRemote command numbers
        let command_num = match action {
            "play" => "0",      // play command
            "pause" => "1",     // pause command
            _ => return Err(format!("Unknown action: {}", action)),
        };

        let script = format!(r#"
            use framework "AppKit"
            try
                set MediaRemote to current application's NSBundle's bundleWithPath:"/System/Library/PrivateFrameworks/MediaRemote.framework/"
                MediaRemote's load()

                set MRNowPlayingController to current application's NSClassFromString("MRNowPlayingController")
                set commandOptions to current application's NSDictionary's alloc()'s init()
                set controller to MRNowPlayingController's localRouteController()

                controller's sendCommand:{} options:commandOptions completion:(missing value)
                return "success"
            on error errMsg
                return "error"
            end try
        "#, command_num);

        match Command::new("osascript").arg("-e").arg(&script).output() {
            Ok(output) => {
                if output.status.success() {
                    let result_str = String::from_utf8_lossy(&output.stdout);
                    let result = result_str.trim();
                    if result == "success" {
                        Ok(format!("MediaRemote {} command sent successfully", action))
                    } else {
                        Err(format!("MediaRemote command failed: {}", result))
                    }
                } else {
                    let error = String::from_utf8_lossy(&output.stderr);
                    Err(format!("MediaRemote command failed: {}", error))
                }
            }
            Err(e) => Err(format!("Failed to execute MediaRemote command: {}", e)),
        }
    }

    pub fn execute_action(&self, action: MusicAction) -> Result<String, String> {
        match action {
            MusicAction::Play => self.play_music(),
            MusicAction::Pause => self.pause_music(),
        }
    }

    fn get_nowplaying_cli_path(&self) -> PathBuf {
        // Try to find the bundled binary relative to the executable
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                // In a bundled app, resources are typically in ../Resources/
                let bundled_path = exe_dir.parent()
                    .map(|p| p.join("Resources").join("nowplaying-cli"))
                    .unwrap_or_else(|| exe_dir.join("nowplaying-cli"));

                if bundled_path.exists() {
                    return bundled_path;
                }

                // Also try directly next to the executable
                let exe_sibling = exe_dir.join("nowplaying-cli");
                if exe_sibling.exists() {
                    return exe_sibling;
                }
            }
        }

        // Try to find it in the build output directory (during development)
        if let Ok(out_dir) = std::env::var("OUT_DIR") {
            let build_path = PathBuf::from(out_dir).join("bin").join("nowplaying-cli");
            if build_path.exists() {
                return build_path;
            }
        }

        // Try local bin directory (for development)
        let local_path = PathBuf::from("bin").join("nowplaying-cli");
        if local_path.exists() {
            return local_path;
        }

        // Final fallback to system PATH
        PathBuf::from("nowplaying-cli")
    }

}

impl Default for MusicController {
    fn default() -> Self {
        Self::new()
    }
}

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
        // Use MediaRemote framework for universal music detection
        let is_playing = self.check_music_via_mediaremote().unwrap_or(false);

        MusicStatus {
            is_playing,
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
        self.send_mediaremote_command("play")
    }

    pub fn pause_music(&self) -> Result<String, String> {
        self.send_mediaremote_command("pause")
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

}

impl Default for MusicController {
    fn default() -> Self {
        Self::new()
    }
}

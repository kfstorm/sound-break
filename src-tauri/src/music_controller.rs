use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicStatus {
    pub is_playing: bool,
    pub player_name: Option<String>,
    pub track_info: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MusicAction {
    Pause,
    Resume,
    Toggle,
}

pub struct MusicController;

impl MusicController {
    pub fn new() -> Self {
        Self
    }

    /// Check if music is currently playing using macOS Media Player API
    pub fn is_music_playing() -> Result<MusicStatus, String> {
        // Check if any music is playing using AppleScript to query the Now Playing info
        let script = r#"
            tell application "System Events"
                try
                    set isPlaying to false
                    set playerName to ""
                    set trackInfo to ""
                    
                    -- Check if Spotify is running without launching it
                    set spotifyRunning to false
                    repeat with proc in (every process whose name is "Spotify")
                        set spotifyRunning to true
                        exit repeat
                    end repeat
                    
                    if spotifyRunning then
                        tell application "Spotify"
                            if player state is playing then
                                set isPlaying to true
                                set playerName to "Spotify"
                                set trackInfo to (artist of current track) & " - " & (name of current track)
                            end if
                        end tell
                    end if
                    
                    -- Check if Music app is running without launching it
                    if not isPlaying then
                        set musicRunning to false
                        repeat with proc in (every process whose name is "Music")
                            set musicRunning to true
                            exit repeat
                        end repeat
                        
                        if musicRunning then
                            tell application "Music"
                                if player state is playing then
                                    set isPlaying to true
                                    set playerName to "Music"
                                    try
                                        set trackInfo to (artist of current track) & " - " & (name of current track)
                                    on error
                                        set trackInfo to "Unknown Track"
                                    end try
                                end if
                            end tell
                        end if
                    end if
                    
                    return (isPlaying as string) & "|" & playerName & "|" & trackInfo
                on error errMsg
                    return "false||Error: " & errMsg
                end try
            end tell
        "#;

        match Command::new("osascript").arg("-e").arg(script).output() {
            Ok(output) => {
                let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
                let parts: Vec<&str> = result.split('|').collect();
                
                if parts.len() >= 3 {
                    let is_playing = parts[0] == "true";
                    let player_name = if parts[1].is_empty() { None } else { Some(parts[1].to_string()) };
                    let track_info = if parts[2].is_empty() { None } else { Some(parts[2].to_string()) };
                    
                    Ok(MusicStatus {
                        is_playing,
                        player_name,
                        track_info,
                    })
                } else {
                    Err("Failed to parse music status".to_string())
                }
            }
            Err(e) => Err(format!("AppleScript execution failed: {}", e)),
        }
    }

    /// Pause all music players
    pub fn pause_music() -> Result<String, String> {
        let script = r#"
            tell application "System Events"
                set pausedApps to {}
                
                -- Check if Spotify is running and pause it
                set spotifyRunning to false
                repeat with proc in (every process whose name is "Spotify")
                    set spotifyRunning to true
                    exit repeat
                end repeat
                
                if spotifyRunning then
                    tell application "Spotify"
                        if player state is playing then
                            pause
                            set pausedApps to pausedApps & {"Spotify"}
                        end if
                    end tell
                end if
                
                -- Check if Music app is running and pause it
                set musicRunning to false
                repeat with proc in (every process whose name is "Music")
                    set musicRunning to true
                    exit repeat
                end repeat
                
                if musicRunning then
                    tell application "Music"
                        if player state is playing then
                            pause
                            set pausedApps to pausedApps & {"Music"}
                        end if
                    end tell
                end if
                
                -- Use media keys as fallback
                if length of pausedApps is 0 then
                    key code 16 using {function down} -- F7 pause key
                    return "Used media key fallback"
                end if
                
                return "Paused: " & (pausedApps as string)
            end tell
        "#;

        match Command::new("osascript").arg("-e").arg(script).output() {
            Ok(output) => {
                let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
                Ok(result)
            }
            Err(e) => Err(format!("Failed to pause music: {}", e)),
        }
    }

    /// Resume music playback
    pub fn resume_music() -> Result<String, String> {
        let script = r#"
            tell application "System Events"
                set resumedApps to {}
                
                -- Check if Spotify is running and resume it
                set spotifyRunning to false
                repeat with proc in (every process whose name is "Spotify")
                    set spotifyRunning to true
                    exit repeat
                end repeat
                
                if spotifyRunning then
                    tell application "Spotify"
                        if player state is paused then
                            play
                            set resumedApps to resumedApps & {"Spotify"}
                        end if
                    end tell
                end if
                
                -- Check if Music app is running and resume it
                set musicRunning to false
                repeat with proc in (every process whose name is "Music")
                    set musicRunning to true
                    exit repeat
                end repeat
                
                if musicRunning then
                    tell application "Music"
                        if player state is paused then
                            play
                            set resumedApps to resumedApps & {"Music"}
                        end if
                    end tell
                end if
                
                -- Use media keys as fallback
                if length of resumedApps is 0 then
                    key code 16 using {function down} -- F7 play/pause key
                    return "Used media key fallback"
                end if
                
                return "Resumed: " & (resumedApps as string)
            end tell
        "#;

        match Command::new("osascript").arg("-e").arg(script).output() {
            Ok(output) => {
                let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
                Ok(result)
            }
            Err(e) => Err(format!("Failed to resume music: {}", e)),
        }
    }

    /// Execute a music action
    pub fn execute_action(action: MusicAction) -> Result<String, String> {
        match action {
            MusicAction::Pause => Self::pause_music(),
            MusicAction::Resume => Self::resume_music(),
            MusicAction::Toggle => {
                // Check current state and toggle
                match Self::is_music_playing() {
                    Ok(status) => {
                        if status.is_playing {
                            Self::pause_music()
                        } else {
                            Self::resume_music()
                        }
                    }
                    Err(_) => Self::pause_music(), // Default to pause if can't determine state
                }
            }
        }
    }
}

impl Default for MusicController {
    fn default() -> Self {
        Self::new()
    }
}

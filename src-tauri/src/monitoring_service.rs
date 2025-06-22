use crate::meeting_detector::{MeetingDetector, MeetingStatus};
use crate::music_controller::{MusicController, MusicAction, MusicStatus};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringStatus {
    pub is_active: bool,
    pub meeting_status: Option<MeetingStatus>,
    pub music_status: Option<MusicStatus>,
    pub last_action: Option<String>,
    pub last_check: u64,
}

pub struct MonitoringService {
    detector: Arc<Mutex<MeetingDetector>>,
    is_running: Arc<Mutex<bool>>,
    was_in_meeting: Arc<Mutex<bool>>,
    music_was_playing_before_meeting: Arc<Mutex<bool>>,
    status: Arc<Mutex<MonitoringStatus>>,
    last_check_time: Arc<Mutex<SystemTime>>,
}

impl MonitoringService {
    pub fn new() -> Self {
        Self {
            detector: Arc::new(Mutex::new(MeetingDetector::new())),
            is_running: Arc::new(Mutex::new(false)),
            was_in_meeting: Arc::new(Mutex::new(false)),
            music_was_playing_before_meeting: Arc::new(Mutex::new(false)),
            status: Arc::new(Mutex::new(MonitoringStatus {
                is_active: false,
                meeting_status: None,
                music_status: None,
                last_action: None,
                last_check: 0,
            })),
            last_check_time: Arc::new(Mutex::new(SystemTime::now())),
        }
    }

    pub fn start_monitoring(&self) -> Result<String, String> {
        let mut is_running = self.is_running.lock().unwrap();
        if *is_running {
            return Ok("Monitoring is already running".to_string());
        }
        *is_running = true;

        // Update status
        {
            let mut status = self.status.lock().unwrap();
            status.is_active = true;
            status.last_action = Some("Monitoring started".to_string());
            status.last_check = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
        }

        Ok("Monitoring started successfully".to_string())
    }

    pub fn stop_monitoring(&self) -> Result<String, String> {
        let mut is_running = self.is_running.lock().unwrap();
        if !*is_running {
            return Ok("Monitoring is not running".to_string());
        }
        *is_running = false;

        // Update status
        {
            let mut status = self.status.lock().unwrap();
            status.is_active = false;
            status.last_action = Some("Monitoring stopped".to_string());
            status.last_check = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
        }

        Ok("Monitoring stopped successfully".to_string())
    }

    pub fn toggle_monitoring(&self) -> Result<String, String> {
        let is_running = *self.is_running.lock().unwrap();
        if is_running {
            self.stop_monitoring()
        } else {
            self.start_monitoring()
        }
    }

    fn perform_monitoring_check(&self) {
        let is_running = *self.is_running.lock().unwrap();
        if !is_running {
            return;
        }

        // Check if enough time has passed since last check (avoid too frequent checks)
        let now = SystemTime::now();
        {
            let mut last_check = self.last_check_time.lock().unwrap();
            if now.duration_since(*last_check).unwrap_or(Duration::from_secs(0)) < Duration::from_secs(1) {
                return; // Too soon since last check
            }
            *last_check = now;
        }

        // Detect meeting status
        let meeting_status = {
            let mut detector = self.detector.lock().unwrap();
            detector.detect_meetings()
        };

        // Check music status
        let music_controller = MusicController::new();
        let music_status = music_controller.get_music_status();

        let now_in_meeting = meeting_status.in_meeting;
        let was_previously_in_meeting = *self.was_in_meeting.lock().unwrap();

        // Handle meeting state transitions
        if now_in_meeting && !was_previously_in_meeting {
            // Entering meeting - pause music if playing
            if music_status.is_playing {
                *self.music_was_playing_before_meeting.lock().unwrap() = true;
                if let Ok(result) = music_controller.execute_action(MusicAction::Pause) {
                    let mut status_guard = self.status.lock().unwrap();
                    status_guard.last_action = Some(format!("Meeting started: {}", result));
                }
            } else {
                *self.music_was_playing_before_meeting.lock().unwrap() = false;
            }
            *self.was_in_meeting.lock().unwrap() = true;
        } else if !now_in_meeting && was_previously_in_meeting {
            // Exiting meeting - resume music if it was playing before
            let should_resume = *self.music_was_playing_before_meeting.lock().unwrap();
            
            if should_resume {
                if let Ok(result) = music_controller.execute_action(MusicAction::Play) {
                    let mut status_guard = self.status.lock().unwrap();
                    status_guard.last_action = Some(format!("Meeting ended: {}", result));
                }
                *self.music_was_playing_before_meeting.lock().unwrap() = false;
            }
            *self.was_in_meeting.lock().unwrap() = false;
        }

        // Update status
        {
            let mut status_guard = self.status.lock().unwrap();
            status_guard.meeting_status = Some(meeting_status);
            status_guard.music_status = Some(music_status);
            status_guard.last_check = now
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
        }
    }

    pub fn get_status(&self) -> MonitoringStatus {
        // Perform a monitoring check each time status is requested
        self.perform_monitoring_check();

        // Return current status
        self.status.lock().unwrap().clone()
    }

    pub fn get_meeting_config(&self) -> crate::meeting_detector::MeetingConfig {
        let detector = self.detector.lock().unwrap();
        detector.get_config().clone()
    }

    pub fn update_meeting_config(&mut self, config: crate::meeting_detector::MeetingConfig) {
        let mut detector = self.detector.lock().unwrap();
        detector.update_config(config);
    }
}

impl Default for MonitoringService {
    fn default() -> Self {
        Self::new()
    }
}

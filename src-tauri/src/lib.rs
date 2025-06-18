mod meeting_detector;
mod music_controller;
mod monitoring_service;

use meeting_detector::MeetingStatus;
use music_controller::{MusicAction, MusicStatus};
use monitoring_service::{MonitoringService, MonitoringStatus};
use std::sync::Mutex;
use tauri::{Manager, menu::{MenuBuilder, MenuItem}, tray::TrayIconBuilder};

// Global monitoring service state
struct AppState {
    monitoring_service: Mutex<MonitoringService>,
}

// Tauri commands
#[tauri::command]
async fn start_monitoring(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let service = state.monitoring_service.lock().unwrap();
    service.start_monitoring()
}

#[tauri::command]
async fn stop_monitoring(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let service = state.monitoring_service.lock().unwrap();
    service.stop_monitoring()
}

#[tauri::command]
async fn toggle_monitoring(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let service = state.monitoring_service.lock().unwrap();
    service.toggle_monitoring()
}

#[tauri::command]
async fn get_monitoring_status(state: tauri::State<'_, AppState>) -> Result<MonitoringStatus, String> {
    let service = state.monitoring_service.lock().unwrap();
    Ok(service.get_status())
}

#[tauri::command]
async fn get_music_status() -> Result<MusicStatus, String> {
    music_controller::MusicController::is_music_playing()
}

#[tauri::command]
async fn control_music(action: String) -> Result<String, String> {
    let music_action = match action.as_str() {
        "pause" => MusicAction::Pause,
        "resume" => MusicAction::Resume,
        "toggle" => MusicAction::Toggle,
        _ => return Err("Invalid music action".to_string()),
    };
    
    music_controller::MusicController::execute_action(music_action)
}

#[tauri::command]
async fn detect_meetings(state: tauri::State<'_, AppState>) -> Result<MeetingStatus, String> {
    let _service = state.monitoring_service.lock().unwrap();
    let mut detector = meeting_detector::MeetingDetector::new();
    Ok(detector.detect_meetings())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState {
        monitoring_service: Mutex::new(MonitoringService::new()),
    };

    tauri::Builder::default()
        .manage(app_state)
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Create tray icon with menu
            let toggle = MenuItem::with_id(app, "toggle", "Toggle Monitoring", true, None::<&str>)?;
            let status = MenuItem::with_id(app, "status", "Show Status", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "Quit SoundBreak", true, None::<&str>)?;
            
            let menu = MenuBuilder::new(app)
                .item(&toggle)
                .item(&status)
                .separator()
                .item(&quit)
                .build()?;
            
            let _tray = TrayIconBuilder::with_id("main")
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .tooltip("SoundBreak - Meeting Music Controller")
                .on_menu_event(move |app, event| match event.id().as_ref() {
                    "toggle" => {
                        let app_state = app.state::<AppState>();
                        let service = app_state.monitoring_service.lock().unwrap();
                        if let Ok(result) = service.toggle_monitoring() {
                            println!("SoundBreak: {}", result);
                        }
                    }
                    "status" => {
                        // Show the main window
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => {
                        std::process::exit(0);
                    }
                    _ => {}
                })
                .build(app)?;
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_monitoring,
            stop_monitoring,
            toggle_monitoring,
            get_monitoring_status,
            get_music_status,
            control_music,
            detect_meetings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

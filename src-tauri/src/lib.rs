mod meeting_detector;
mod music_controller;
mod monitoring_service;

use meeting_detector::{MeetingStatus, MeetingConfig};
use music_controller::{MusicAction, MusicStatus};
use monitoring_service::{MonitoringService, MonitoringStatus};
use std::sync::Mutex;
use tauri::{Manager, menu::{MenuBuilder, MenuItem}, tray::TrayIconBuilder};

// Global monitoring service state
struct AppState {
    monitoring_service: Mutex<MonitoringService>,
}

// Helper function to update tray menu with current status
fn update_tray_menu(app: &tauri::AppHandle, state: &AppState) -> Result<(), tauri::Error> {
    let service = state.monitoring_service.lock().unwrap();
    let status = service.get_status();

    let monitoring_status_text = if status.is_active {
        "✅ Monitoring Active"
    } else {
        "⏸️ Monitoring Stopped"
    };

    let music_status_text = match &status.music_status {
        Some(music) if music.is_playing => "🎵 Music Playing",
        Some(_) => "⏸️ Music Paused",
        None => "❓ Music Status Unknown",
    };

    let meeting_status_text = match &status.meeting_status {
        Some(meeting) if meeting.in_meeting => "🎤 In Meeting",
        Some(_) => "📵 Not in Meeting",
        None => "❓ Meeting Status Unknown",
    };

    let toggle_text = if status.is_active {
        "⏸️ Stop Monitoring"
    } else {
        "▶️ Start Monitoring"
    };

    // Create menu items
    let monitoring_status = MenuItem::with_id(app, "monitoring_status", monitoring_status_text, false, None::<&str>)?;
    let music_status = MenuItem::with_id(app, "music_status", music_status_text, false, None::<&str>)?;
    let meeting_status = MenuItem::with_id(app, "meeting_status", meeting_status_text, false, None::<&str>)?;
    let toggle = MenuItem::with_id(app, "toggle", toggle_text, true, None::<&str>)?;
    let show_window = MenuItem::with_id(app, "show_window", "Show SoundBreak", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Quit SoundBreak", true, None::<&str>)?;

    let menu = MenuBuilder::new(app)
        .item(&monitoring_status)
        .item(&music_status)
        .item(&meeting_status)
        .separator()
        .item(&toggle)
        .item(&show_window)
        .separator()
        .item(&quit)
        .build()?;

    if let Some(tray) = app.tray_by_id("main") {
        tray.set_menu(Some(menu))?;
    }

    Ok(())
}

// Tauri commands
#[tauri::command]
async fn start_monitoring(app: tauri::AppHandle, state: tauri::State<'_, AppState>) -> Result<String, String> {
    let service = state.monitoring_service.lock().unwrap();
    let result = service.start_monitoring();
    drop(service); // Release the lock before updating tray
    let _ = update_tray_menu(&app, &state);
    result
}

#[tauri::command]
async fn stop_monitoring(app: tauri::AppHandle, state: tauri::State<'_, AppState>) -> Result<String, String> {
    let service = state.monitoring_service.lock().unwrap();
    let result = service.stop_monitoring();
    drop(service); // Release the lock before updating tray
    let _ = update_tray_menu(&app, &state);
    result
}

#[tauri::command]
async fn toggle_monitoring(app: tauri::AppHandle, state: tauri::State<'_, AppState>) -> Result<String, String> {
    let service = state.monitoring_service.lock().unwrap();
    let result = service.toggle_monitoring();
    drop(service); // Release the lock before updating tray
    let _ = update_tray_menu(&app, &state);
    result
}

#[tauri::command]
async fn get_monitoring_status(state: tauri::State<'_, AppState>) -> Result<MonitoringStatus, String> {
    let service = state.monitoring_service.lock().unwrap();
    Ok(service.get_status())
}

#[tauri::command]
async fn get_music_status() -> Result<MusicStatus, String> {
    let controller = music_controller::MusicController::new();
    Ok(controller.get_music_status())
}

#[tauri::command]
async fn control_music(action: String) -> Result<String, String> {
    let music_action = match action.as_str() {
        "play" => MusicAction::Play,
        "pause" => MusicAction::Pause,
        _ => return Err("Invalid music action. Only 'play' and 'pause' are supported.".to_string()),
    };

    let controller = music_controller::MusicController::new();
    controller.execute_action(music_action)
}

#[tauri::command]
async fn detect_meetings(state: tauri::State<'_, AppState>) -> Result<MeetingStatus, String> {
    let _service = state.monitoring_service.lock().unwrap();
    let mut detector = meeting_detector::MeetingDetector::new();
    Ok(detector.detect_meetings())
}

#[tauri::command]
async fn get_meeting_config(state: tauri::State<'_, AppState>) -> Result<MeetingConfig, String> {
    let service = state.monitoring_service.lock().unwrap();
    Ok(service.get_meeting_config())
}

#[tauri::command]
async fn update_meeting_config(state: tauri::State<'_, AppState>, config: MeetingConfig) -> Result<String, String> {
    let mut service = state.monitoring_service.lock().unwrap();
    service.update_meeting_config(config);
    Ok("Meeting configuration updated successfully".to_string())
}

#[tauri::command]
async fn refresh_tray_menu(app: tauri::AppHandle, state: tauri::State<'_, AppState>) -> Result<String, String> {
    update_tray_menu(&app, &state).map_err(|e| e.to_string())?;
    Ok("Tray menu updated".to_string())
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
            let app_state = app.state::<AppState>();

            // Start monitoring automatically on startup
            {
                let service = app_state.monitoring_service.lock().unwrap();
                if let Ok(result) = service.start_monitoring() {
                    println!("SoundBreak: Auto-started monitoring - {}", result);
                }
            }

            // Create initial tray menu (will be updated with status shortly)
            let toggle = MenuItem::with_id(app, "toggle", "▶️ Start Monitoring", true, None::<&str>)?;
            let show_window = MenuItem::with_id(app, "show_window", "Show SoundBreak", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "Quit SoundBreak", true, None::<&str>)?;

            let menu = MenuBuilder::new(app)
                .item(&toggle)
                .item(&show_window)
                .separator()
                .item(&quit)
                .build()?;

            let app_handle = app.handle().clone();
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
                            drop(service); // Release lock before updating tray
                            let _ = update_tray_menu(app, &app_state);
                        }
                    }
                    "show_window" => {
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

            // Set up window close event to hide instead of close
            if let Some(window) = app.get_webview_window("main") {
                let window_clone = window.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close();
                        let _ = window_clone.hide();
                    }
                });
            }

            // Update tray menu with initial status after a short delay to allow monitoring to start
            let app_handle_clone = app_handle.clone();
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(500));
                let app_state = app_handle_clone.state::<AppState>();
                let _ = update_tray_menu(&app_handle_clone, &app_state);

                // Start periodic tray menu updates every 2 seconds while monitoring is active
                loop {
                    std::thread::sleep(std::time::Duration::from_secs(2));
                    let app_state = app_handle_clone.state::<AppState>();
                    let is_monitoring = {
                        let service = app_state.monitoring_service.lock().unwrap();
                        service.get_status().is_active
                    };

                    if is_monitoring {
                        let _ = update_tray_menu(&app_handle_clone, &app_state);
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_monitoring,
            stop_monitoring,
            toggle_monitoring,
            get_monitoring_status,
            get_music_status,
            control_music,
            detect_meetings,
            get_meeting_config,
            update_meeting_config,
            refresh_tray_menu
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

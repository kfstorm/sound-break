mod meeting_detector;
mod music_controller;
mod monitoring_service;

use meeting_detector::{MeetingStatus, MeetingConfig};
use music_controller::{MusicAction, MusicStatus};
use monitoring_service::{MonitoringService, MonitoringStatus};
use std::sync::Mutex;
use tauri::{Manager, menu::{MenuBuilder, MenuItem}, tray::{TrayIcon, TrayIconBuilder}};

// Global monitoring service state
struct AppState {
    monitoring_service: Mutex<MonitoringService>,
    tray_icon: Mutex<Option<TrayIcon>>,
    last_status: Mutex<Option<MonitoringStatus>>,
    // Store menu item references for efficient updates
    monitoring_status_item: Mutex<Option<MenuItem<tauri::Wry>>>,
    music_status_item: Mutex<Option<MenuItem<tauri::Wry>>>,
    meeting_status_item: Mutex<Option<MenuItem<tauri::Wry>>>,
    toggle_item: Mutex<Option<MenuItem<tauri::Wry>>>,
}

// Helper function to check if status has changed significantly
fn has_status_changed(old_status: &Option<MonitoringStatus>, new_status: &MonitoringStatus) -> bool {
    match old_status {
        None => true, // First time, always update
        Some(old) => {
            // Check if any significant fields have changed
            old.is_active != new_status.is_active ||
            old.meeting_status.as_ref().map(|m| m.in_meeting) != new_status.meeting_status.as_ref().map(|m| m.in_meeting) ||
            old.music_status.as_ref().map(|m| m.is_playing) != new_status.music_status.as_ref().map(|m| m.is_playing)
        }
    }
}

// Helper function to update tray menu with current status (using set_text on existing items)
fn update_tray_menu_text(app: &tauri::AppHandle, status: &MonitoringStatus) -> Result<(), Box<dyn std::error::Error>> {
    let app_state = app.state::<AppState>();

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

    // Update menu item texts using set_text()
    if let Some(item) = app_state.monitoring_status_item.lock().unwrap().as_ref() {
        item.set_text(monitoring_status_text)?;
    }

    if let Some(item) = app_state.music_status_item.lock().unwrap().as_ref() {
        item.set_text(music_status_text)?;
    }

    if let Some(item) = app_state.meeting_status_item.lock().unwrap().as_ref() {
        item.set_text(meeting_status_text)?;
    }

    if let Some(item) = app_state.toggle_item.lock().unwrap().as_ref() {
        item.set_text(toggle_text)?;
    }

    Ok(())
}

// Tauri commands
#[tauri::command]
async fn start_monitoring(app: tauri::AppHandle, state: tauri::State<'_, AppState>) -> Result<String, String> {
    let result = {
        let service = state.monitoring_service.lock().unwrap();
        service.start_monitoring()
    };

    // Update tray menu after starting monitoring
    if result.is_ok() {
        let status = {
            let service = state.monitoring_service.lock().unwrap();
            service.get_status()
        };
        let _ = update_tray_menu_text(&app, &status);
    }

    result
}

#[tauri::command]
async fn stop_monitoring(app: tauri::AppHandle, state: tauri::State<'_, AppState>) -> Result<String, String> {
    let result = {
        let service = state.monitoring_service.lock().unwrap();
        service.stop_monitoring()
    };

    // Update tray menu after stopping monitoring
    if result.is_ok() {
        let status = {
            let service = state.monitoring_service.lock().unwrap();
            service.get_status()
        };
        let _ = update_tray_menu_text(&app, &status);
    }

    result
}

#[tauri::command]
async fn toggle_monitoring(app: tauri::AppHandle, state: tauri::State<'_, AppState>) -> Result<String, String> {
    let result = {
        let service = state.monitoring_service.lock().unwrap();
        service.toggle_monitoring()
    };

    // Update tray menu after toggling monitoring
    if result.is_ok() {
        let status = {
            let service = state.monitoring_service.lock().unwrap();
            service.get_status()
        };
        let _ = update_tray_menu_text(&app, &status);
    }

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
    let status = {
        let service = state.monitoring_service.lock().unwrap();
        service.get_status()
    };
    update_tray_menu_text(&app, &status).map_err(|e| e.to_string())?;
    Ok("Tray menu updated".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState {
        monitoring_service: Mutex::new(MonitoringService::new()),
        tray_icon: Mutex::new(None),
        last_status: Mutex::new(None),
        monitoring_status_item: Mutex::new(None),
        music_status_item: Mutex::new(None),
        meeting_status_item: Mutex::new(None),
        toggle_item: Mutex::new(None),
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

            // Create initial tray menu with all items
            let monitoring_status = MenuItem::with_id(app, "monitoring_status", "⏸️ Monitoring Stopped", false, None::<&str>)?;
            let music_status = MenuItem::with_id(app, "music_status", "❓ Music Status Unknown", false, None::<&str>)?;
            let meeting_status = MenuItem::with_id(app, "meeting_status", "❓ Meeting Status Unknown", false, None::<&str>)?;
            let toggle = MenuItem::with_id(app, "toggle", "▶️ Start Monitoring", true, None::<&str>)?;
            let show_window = MenuItem::with_id(app, "show_window", "Show SoundBreak", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "Quit SoundBreak", true, None::<&str>)?;

            // Store menu item references for later updates
            {
                *app_state.monitoring_status_item.lock().unwrap() = Some(monitoring_status.clone());
                *app_state.music_status_item.lock().unwrap() = Some(music_status.clone());
                *app_state.meeting_status_item.lock().unwrap() = Some(meeting_status.clone());
                *app_state.toggle_item.lock().unwrap() = Some(toggle.clone());
            }

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

            let app_handle = app.handle().clone();
            let tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .tooltip("SoundBreak - Meeting Music Controller")
                .on_menu_event(move |app, event| match event.id().as_ref() {
                    "toggle" => {
                        let app_state = app.state::<AppState>();
                        let result = {
                            let service = app_state.monitoring_service.lock().unwrap();
                            service.toggle_monitoring()
                        };
                        if let Ok(msg) = result {
                            println!("SoundBreak: {}", msg);
                            // Update tray menu after toggling
                            let status = {
                                let service = app_state.monitoring_service.lock().unwrap();
                                service.get_status()
                            };
                            let _ = update_tray_menu_text(app, &status);
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

            // Store the tray icon reference
            {
                let mut tray_guard = app_state.tray_icon.lock().unwrap();
                *tray_guard = Some(tray);
            }

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
                let status = {
                    let service = app_state.monitoring_service.lock().unwrap();
                    service.get_status()
                };
                let _ = update_tray_menu_text(&app_handle_clone, &status);

                // Store initial status
                {
                    let mut last_status = app_state.last_status.lock().unwrap();
                    *last_status = Some(status);
                }

                // Start periodic status check every 2 seconds
                loop {
                    std::thread::sleep(std::time::Duration::from_secs(2));

                    let current_status = {
                        let service = app_state.monitoring_service.lock().unwrap();
                        service.get_status()
                    };

                    // Check if status has changed
                    let should_update = {
                        let last_status = app_state.last_status.lock().unwrap();
                        has_status_changed(&*last_status, &current_status)
                    };

                    if should_update {
                        let _ = update_tray_menu_text(&app_handle_clone, &current_status);
                        // Update stored status
                        let mut last_status = app_state.last_status.lock().unwrap();
                        *last_status = Some(current_status);
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

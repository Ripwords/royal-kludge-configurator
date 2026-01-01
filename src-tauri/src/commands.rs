use crate::hid::HidManager;
use crate::models::{Keyboard, KeyboardConfig, Mode};
use crate::modes::{get_rgb_modes, get_single_color_modes};
use crate::protocol::build_buffers;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, State};

/// Global HID manager state
pub type HidManagerState = Arc<Mutex<HidManager>>;

/// Scan for connected keyboards
#[tauri::command]
pub fn scan_keyboards(
    _app: AppHandle,
    hid_manager: State<'_, HidManagerState>,
) -> Result<Vec<Keyboard>, String> {
    let manager = hid_manager.lock().unwrap();
    manager.scan_keyboards()
}

/// Send keyboard configuration to device
#[tauri::command]
pub fn send_keyboard_config(
    keyboard_path: String,
    config: KeyboardConfig,
    keyboard: Keyboard,
    hid_manager: State<'_, HidManagerState>,
) -> Result<(), String> {
    let manager = hid_manager.lock().unwrap();

    // Build protocol buffers
    let buffers = build_buffers(&keyboard, &config)
        .map_err(|e| format!("Failed to build protocol buffers: {}", e))?;

    // Send buffers to device
    manager
        .send_buffers(&keyboard_path, &buffers)
        .map_err(|e| format!("Failed to send configuration: {}", e))?;

    Ok(())
}

// Database operations are handled in the frontend using tauri-plugin-sql JavaScript API

/// Initialize HID manager (called on app startup)
pub fn init_hid_manager() -> Result<HidManagerState, String> {
    let manager = HidManager::new()?;
    Ok(Arc::new(Mutex::new(manager)))
}

/// Get available lighting modes for a keyboard
#[tauri::command]
pub fn get_lighting_modes(is_rgb: bool) -> Vec<Mode> {
    if is_rgb {
        get_rgb_modes()
    } else {
        get_single_color_modes()
    }
}

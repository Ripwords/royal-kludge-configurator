use crate::models::{Key, KeyCode, Keyboard, KeyboardUsbId};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

/// Load keyboard configuration from JSON file
pub fn load_keyboard_config(
    vid: u16,
    pid: u16,
    device_path: &str,
    resource_dir: &Path,
) -> Result<Keyboard, String> {
    let config_path = resource_dir
        .join(format!("{:x}", vid))
        .join("configs")
        .join(format!("{:x}.json", pid));
    
    if !config_path.exists() {
        return Err(format!("Config file not found: {}", config_path.display()));
    }

    let config_content = fs::read_to_string(&config_path)
        .map_err(|e| format!("Failed to read config file: {}", e))?;

    let config: Value = serde_json::from_str(&config_content)
        .map_err(|e| format!("Failed to parse config JSON: {}", e))?;

    if !config["enabled"].as_bool().unwrap_or(false) {
        return Err("Keyboard is disabled in config".to_string());
    }

    let name = config["name"]
        .as_str()
        .ok_or("Missing 'name' field")?
        .to_string();

    let top = config["top"].as_array().ok_or("Missing 'top' array")?;
    let bottom = config["bottom"]
        .as_array()
        .ok_or("Missing 'bottom' array")?;

    let top_left_x = top[0].as_i64().ok_or("Invalid top[0]")? as i32;
    let top_left_y = top[1].as_i64().ok_or("Invalid top[1]")? as i32;
    let bottom_right_x = bottom[0].as_i64().ok_or("Invalid bottom[0]")? as i32;
    let bottom_right_y = bottom[1].as_i64().ok_or("Invalid bottom[1]")? as i32;

    let key_map_enabled = config["keyMapEnabled"].as_bool().unwrap_or(false);
    let light_enabled = config["lightEnabled"].as_bool().unwrap_or(false);
    let rgb = config["rgb"].as_bool().unwrap_or(false);

    // Debug log for RGB status
    eprintln!(
        "[RK-Configurator] Loaded keyboard config: name={}, rgb={}",
        name, rgb
    );

    let mut keys = Vec::new();

    if key_map_enabled {
        let keys_array = config["keys"].as_array().ok_or("Missing 'keys' array")?;

        for key_obj in keys_array {
            let buffer_index = key_obj["bIndex"]
                .as_u64()
                .ok_or("Missing or invalid 'bIndex'")? as u8;

            let key_code_str = key_obj["keyCode"].as_str().ok_or("Missing 'keyCode'")?;

            let key_code = KeyCode::from_str(key_code_str)
                .ok_or_else(|| format!("Unknown key code: {}", key_code_str))?;

            if key_code == KeyCode::KeyInvalid {
                continue;
            }

            let key_top = key_obj["top"].as_array().ok_or("Missing key 'top' array")?;
            let key_bottom = key_obj["bottom"]
                .as_array()
                .ok_or("Missing key 'bottom' array")?;

            let key_top_x = key_top[0].as_i64().ok_or("Invalid key top[0]")? as i32;
            let key_top_y = key_top[1].as_i64().ok_or("Invalid key top[1]")? as i32;
            let key_bottom_x = key_bottom[0].as_i64().ok_or("Invalid key bottom[0]")? as i32;
            let key_bottom_y = key_bottom[1].as_i64().ok_or("Invalid key bottom[1]")? as i32;

            keys.push(Key {
                buffer_index,
                key_code,
                top_x: key_top_x,
                top_y: key_top_y,
                bottom_x: key_bottom_x,
                bottom_y: key_bottom_y,
            });
        }
    }

    let image_path = format!("keyboards/{:x}/images/{:x}.png", vid, pid);

    // Verify image exists
    let image_full_path = resource_dir
        .join(format!("{:x}", vid))
        .join("images")
        .join(format!("{:x}.png", pid));

    if !image_full_path.exists() {
        return Err(format!(
            "Keyboard image not found: {}",
            image_full_path.display()
        ));
    }

    Ok(Keyboard {
        id: KeyboardUsbId { vid, pid },
        path: device_path.to_string(),
        name,
        image_path,
        keys,
        key_map_enabled,
        light_enabled,
        rgb,
        top_left_x,
        top_left_y,
        bottom_right_x,
        bottom_right_y,
    })
}

/// Check if a device path matches Windows HID collection pattern
#[cfg(windows)]
pub fn matches_windows_path(_path: &str, _vid: u16, _pid: u16, col_number: Option<&str>) -> bool {
    if let Some(col) = col_number {
        let pattern = format!("&Col{}", col);
        _path.contains(&pattern)
    } else {
        // Default collection check - try common collection numbers
        _path.contains("&Col01") || _path.contains("&Col02")
    }
}

/// Check if device matches Linux/Mac criteria (usage page and usage)
#[cfg(not(windows))]
pub fn matches_unix_device(usage_page: u16, usage: u16) -> bool {
    usage_page == 0x0001 && usage == 0x0080
}

/// Get resource directory path
pub fn get_resource_dir() -> PathBuf {
    // For development, look for keyboards relative to the executable or in src-tauri
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            // On macOS, resources are in .app/Contents/Resources
            #[cfg(target_os = "macos")]
            {
                let resources_path = exe_dir.join("..").join("Resources").join("keyboards");
                if let Ok(canonical) = resources_path.canonicalize() {
                    if canonical.exists() {
                        eprintln!("[Resource] Using macOS Resources dir: {}", canonical.display());
                        return canonical;
                    }
                }
                // Also try .app/Contents/Resources directly
                let app_resources = exe_dir.join("..").join("Resources");
                if let Ok(canonical) = app_resources.canonicalize() {
                    eprintln!("[Resource] Checking macOS Resources: {}", canonical.display());
                    let keyboards_in_resources = canonical.join("keyboards");
                    if keyboards_in_resources.exists() {
                        eprintln!("[Resource] Found keyboards in Resources: {}", keyboards_in_resources.display());
                        return keyboards_in_resources;
                    }
                }
            }
            
            // Try relative to executable first (for built app)
            let relative_path = exe_dir.join("keyboards");
            if relative_path.exists() {
                eprintln!("[Resource] Using relative to exe: {}", relative_path.display());
                return relative_path;
            }
            
            // Try in parent directory (for development)
            if let Some(parent) = exe_dir.parent() {
                let dev_path = parent.join("keyboards");
                if dev_path.exists() {
                    eprintln!("[Resource] Using parent dir: {}", dev_path.display());
                    return dev_path;
                }
                // Try src-tauri/keyboards (for development)
                let src_tauri_path = parent.join("src-tauri").join("keyboards");
                if src_tauri_path.exists() {
                    eprintln!("[Resource] Using src-tauri dir: {}", src_tauri_path.display());
                    return src_tauri_path;
                }
            }
        }
    }

    // Fallback: try current directory or src-tauri/keyboards
    let current_keyboards = PathBuf::from("keyboards");
    if current_keyboards.exists() {
        eprintln!("[Resource] Using current dir: {}", current_keyboards.display());
        return current_keyboards;
    }

    let src_tauri_keyboards = PathBuf::from("src-tauri/keyboards");
    if src_tauri_keyboards.exists() {
        eprintln!("[Resource] Using src-tauri fallback: {}", src_tauri_keyboards.display());
        return src_tauri_keyboards;
    }

    // Last resort: return a path that might work
    eprintln!("[Resource] WARNING: Could not find keyboards directory, using fallback");
    PathBuf::from("keyboards")
}

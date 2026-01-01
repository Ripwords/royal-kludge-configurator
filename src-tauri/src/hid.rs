use crate::keyboard::{get_resource_dir, load_keyboard_config, matches_unix_device};
use crate::models::Keyboard;
use hidapi::HidApi;
use std::sync::{Arc, Mutex};

const SUPPORTED_VID: u16 = 0x258a;
const BUFFER_SIZE: usize = 65;

/// HID manager for device enumeration and communication
pub struct HidManager {
    api: Arc<Mutex<HidApi>>,
}

impl HidManager {
    /// Initialize HID API
    pub fn new() -> Result<Self, String> {
        let api = HidApi::new().map_err(|e| format!("Failed to initialize HID API: {}", e))?;

        Ok(Self {
            api: Arc::new(Mutex::new(api)),
        })
    }

    /// Scan for connected keyboards
    pub fn scan_keyboards(&self) -> Result<Vec<Keyboard>, String> {
        // Refresh HID API to detect newly connected devices
        let mut api_guard = self.api.lock().unwrap();
        *api_guard = HidApi::new()
            .map_err(|e| format!("Failed to refresh HID API: {}", e))?;
        let api = api_guard;
        
        let resource_dir = get_resource_dir();
        let devices = api.device_list();
        
        let mut keyboards = Vec::new();
        let mut seen_ids = std::collections::HashSet::new();

        for device_info in devices {
            let vid = device_info.vendor_id();
            let pid = device_info.product_id();

            // Only process devices with supported VID
            if vid != SUPPORTED_VID {
                continue;
            }

            // Skip if we've already processed this VID/PID combination
            let usb_id = (vid, pid);
            if seen_ids.contains(&usb_id) {
                continue;
            }

            let device_path = device_info.path().to_string_lossy().to_string();

            // Platform-specific filtering
            #[cfg(windows)]
            {
                // On Windows, we need to check the path for collection number
                // For now, we'll try common collection numbers (01, 02)
                // In a full implementation, we'd load hid-col.json to get collection numbers
                if !device_path.contains("&Col01") && !device_path.contains("&Col02") {
                    continue;
                }
            }

            #[cfg(not(windows))]
            {
                // On Linux/Mac, check usage page and usage
                let usage_page = device_info.usage_page();
                let usage = device_info.usage();
                if !matches_unix_device(usage_page, usage) {
                    continue;
                }
            }

            // Try to load keyboard config
            match load_keyboard_config(vid, pid, &device_path, &resource_dir) {
                Ok(keyboard) => {
                    seen_ids.insert(usb_id);
                    keyboards.push(keyboard);
                }
                Err(_) => {
                    // Device might not be supported, continue
                }
            }
        }

        Ok(keyboards)
    }

    /// Send feature report buffers to keyboard
    pub fn send_buffers(&self, device_path: &str, buffers: &[Vec<u8>]) -> Result<(), String> {
        let api = self.api.lock().unwrap();

        // Convert string path to CString for hidapi
        use std::ffi::CString;
        let c_path =
            CString::new(device_path).map_err(|e| format!("Invalid device path: {}", e))?;

        let device = api
            .open_path(c_path.as_c_str())
            .map_err(|e| format!("Failed to open device at {}: {}", device_path, e))?;

        for (i, buffer) in buffers.iter().enumerate() {
            if buffer.len() != BUFFER_SIZE {
                return Err(format!(
                    "Buffer {} has incorrect size: expected {}, got {}",
                    i,
                    BUFFER_SIZE,
                    buffer.len()
                ));
            }

            device
                .send_feature_report(buffer)
                .map_err(|e| format!("Failed to send feature report {}: {}", i, e))?;
        }

        Ok(())
    }
}

impl Default for HidManager {
    fn default() -> Self {
        Self::new().expect("Failed to initialize HID manager")
    }
}

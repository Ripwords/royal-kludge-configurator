use serde::{Deserialize, Serialize};

/// USB Vendor ID and Product ID pair
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KeyboardUsbId {
    pub vid: u16,
    pub pid: u16,
}

/// Key code enum matching the protocol codes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum KeyCode {
    // Numbers
    Key1 = 0x1e00,
    Key2 = 0x1f00,
    Key3 = 0x2000,
    Key4 = 0x2100,
    Key5 = 0x2200,
    Key6 = 0x2300,
    Key7 = 0x2400,
    Key8 = 0x2500,
    Key9 = 0x2600,
    Key0 = 0x2700,

    // Letters
    KeyA = 0x0400,
    KeyB = 0x0500,
    KeyC = 0x0600,
    KeyD = 0x0700,
    KeyE = 0x0800,
    KeyF = 0x0900,
    KeyG = 0x0a00,
    KeyH = 0x0b00,
    KeyI = 0x0c00,
    KeyJ = 0x0d00,
    KeyK = 0x0e00,
    KeyL = 0x0f00,
    KeyM = 0x1000,
    KeyN = 0x1100,
    KeyO = 0x1200,
    KeyP = 0x1300,
    KeyQ = 0x1400,
    KeyR = 0x1500,
    KeyS = 0x1600,
    KeyT = 0x1700,
    KeyU = 0x1800,
    KeyV = 0x1900,
    KeyW = 0x1a00,
    KeyX = 0x1b00,
    KeyY = 0x1c00,
    KeyZ = 0x1d00,

    // Symbols
    KeyHyphen = 0x2d00,
    KeyEquals = 0x2e00,
    KeyLeftBracket = 0x2f00,
    KeyRightBracket = 0x3000,
    KeyBackSlash = 0x3100,
    KeySemiColon = 0x3300,
    KeyQuote = 0x3400,
    KeyBackQuote = 0x3500,
    KeyComma = 0x3600,
    KeyDot = 0x3700,
    KeySlash = 0x3800,

    // Modifiers
    KeyLeftControl = 0x010000,
    KeyLeftShift = 0x020000,
    KeyLeftAlt = 0x040000,
    KeyRightControl = 0x100000,
    KeyRightShift = 0x200000,
    KeyRightAlt = 0x400000,
    KeyLeftSuper = 0x080000,
    KeyRightSuper = 0x800000,

    // Arrow keys
    KeyRight = 0x4f00,
    KeyLeft = 0x5000,
    KeyDown = 0x5100,
    KeyUp = 0x5200,

    // Function keys
    KeyF1 = 0x3a00,
    KeyF2 = 0x3b00,
    KeyF3 = 0x3c00,
    KeyF4 = 0x3d00,
    KeyF5 = 0x3e00,
    KeyF6 = 0x3f00,
    KeyF7 = 0x4000,
    KeyF8 = 0x4100,
    KeyF9 = 0x4200,
    KeyF10 = 0x4300,
    KeyF11 = 0x4400,
    KeyF12 = 0x4500,

    // Special keys
    KeyEscape = 0x2900,
    KeyTab = 0x2b00,
    KeyEnter = 0x2800,
    KeyMenu = 0x6500,
    KeyInsert = 0x4900,
    KeyPause = 0x4800,
    KeySpace = 0x2c00,
    KeyHome = 0x4a00,
    KeyEnd = 0x4d00,
    KeyDelete = 0x4c00,
    KeyPageDown = 0x4e00,
    KeyPageUp = 0x4b00,
    KeyCapsLock = 0x3900,
    KeyBackspace = 0x2a00,
    KeyPrintScreen = 0x4600,

    // Numpad
    KeyNum1 = 0x5900,
    KeyNum2 = 0x5a00,
    KeyNum3 = 0x5b00,
    KeyNum4 = 0x5c00,
    KeyNum5 = 0x5d00,
    KeyNum6 = 0x5e00,
    KeyNum7 = 0x5f00,
    KeyNum8 = 0x6000,
    KeyNum9 = 0x6100,
    KeyNum0 = 0x6200,
    KeyNumAdd = 0x5700,
    KeyNumSubstract = 0x5600,
    KeyNumMultiply = 0x5500,
    KeyNumDivide = 0x5400,
    KeyNumDecimalPoint = 0x6300,
    KeyNumLock = 0x5300,
    KeyNumEnter = 0x5800,

    // Function key
    KeyFn = 0xb000,

    // Shortcuts
    ShortcutExplorer = 0x01000194,
    ShortcutCut = 0x011b00,
    ShortcutCopy = 0x010600,
    ShortcutPaste = 0x011900,
    ShortcutSave = 0x011600,
    ShortcutShowDesktop = 0x080700,
    ShortcutLock = 0x080f00,
    ShortcutSwitchWindow = 0x042b00,
    ShortcutCloseWindow = 0x043d00,

    // Multimedia
    MultimediaPlayPause = 0x010000cd,
    MultimediaStop = 0x010000b7,
    MultimediaPrevious = 0x010000b6,
    MultimediaNext = 0x010000b5,
    MultimediaVolumeUp = 0x010000e9,
    MultimediaVolumeDown = 0x010000ea,
    MultimediaMute = 0x010000e2,

    // Invalid
    KeyInvalid = 0,
}

impl KeyCode {
    /// Parse key code from string (e.g., "Key_A", "Key_1")
    pub fn from_str(s: &str) -> Option<KeyCode> {
        match s {
            "Key_1" => Some(KeyCode::Key1),
            "Key_2" => Some(KeyCode::Key2),
            "Key_3" => Some(KeyCode::Key3),
            "Key_4" => Some(KeyCode::Key4),
            "Key_5" => Some(KeyCode::Key5),
            "Key_6" => Some(KeyCode::Key6),
            "Key_7" => Some(KeyCode::Key7),
            "Key_8" => Some(KeyCode::Key8),
            "Key_9" => Some(KeyCode::Key9),
            "Key_0" => Some(KeyCode::Key0),
            "Key_A" => Some(KeyCode::KeyA),
            "Key_B" => Some(KeyCode::KeyB),
            "Key_C" => Some(KeyCode::KeyC),
            "Key_D" => Some(KeyCode::KeyD),
            "Key_E" => Some(KeyCode::KeyE),
            "Key_F" => Some(KeyCode::KeyF),
            "Key_G" => Some(KeyCode::KeyG),
            "Key_H" => Some(KeyCode::KeyH),
            "Key_I" => Some(KeyCode::KeyI),
            "Key_J" => Some(KeyCode::KeyJ),
            "Key_K" => Some(KeyCode::KeyK),
            "Key_L" => Some(KeyCode::KeyL),
            "Key_M" => Some(KeyCode::KeyM),
            "Key_N" => Some(KeyCode::KeyN),
            "Key_O" => Some(KeyCode::KeyO),
            "Key_P" => Some(KeyCode::KeyP),
            "Key_Q" => Some(KeyCode::KeyQ),
            "Key_R" => Some(KeyCode::KeyR),
            "Key_S" => Some(KeyCode::KeyS),
            "Key_T" => Some(KeyCode::KeyT),
            "Key_U" => Some(KeyCode::KeyU),
            "Key_V" => Some(KeyCode::KeyV),
            "Key_W" => Some(KeyCode::KeyW),
            "Key_X" => Some(KeyCode::KeyX),
            "Key_Y" => Some(KeyCode::KeyY),
            "Key_Z" => Some(KeyCode::KeyZ),
            "Key_Hyphen" => Some(KeyCode::KeyHyphen),
            "Key_Equals" => Some(KeyCode::KeyEquals),
            "Key_Left_Bracket" => Some(KeyCode::KeyLeftBracket),
            "Key_Right_Bracket" => Some(KeyCode::KeyRightBracket),
            "Key_Back_Slash" => Some(KeyCode::KeyBackSlash),
            "Key_Semi_Colon" => Some(KeyCode::KeySemiColon),
            "Key_Quote" => Some(KeyCode::KeyQuote),
            "Key_Back_Quote" => Some(KeyCode::KeyBackQuote),
            "Key_Comma" => Some(KeyCode::KeyComma),
            "Key_Dot" => Some(KeyCode::KeyDot),
            "Key_Slash" => Some(KeyCode::KeySlash),
            "Key_Left_Control" => Some(KeyCode::KeyLeftControl),
            "Key_Left_Shift" => Some(KeyCode::KeyLeftShift),
            "Key_Left_Alt" => Some(KeyCode::KeyLeftAlt),
            "Key_Right_Control" => Some(KeyCode::KeyRightControl),
            "Key_Right_Shift" => Some(KeyCode::KeyRightShift),
            "Key_Right_Alt" => Some(KeyCode::KeyRightAlt),
            "Key_Left_Super" => Some(KeyCode::KeyLeftSuper),
            "Key_Right_Super" => Some(KeyCode::KeyRightSuper),
            "Key_Right" => Some(KeyCode::KeyRight),
            "Key_Left" => Some(KeyCode::KeyLeft),
            "Key_Down" => Some(KeyCode::KeyDown),
            "Key_Up" => Some(KeyCode::KeyUp),
            "Key_F1" => Some(KeyCode::KeyF1),
            "Key_F2" => Some(KeyCode::KeyF2),
            "Key_F3" => Some(KeyCode::KeyF3),
            "Key_F4" => Some(KeyCode::KeyF4),
            "Key_F5" => Some(KeyCode::KeyF5),
            "Key_F6" => Some(KeyCode::KeyF6),
            "Key_F7" => Some(KeyCode::KeyF7),
            "Key_F8" => Some(KeyCode::KeyF8),
            "Key_F9" => Some(KeyCode::KeyF9),
            "Key_F10" => Some(KeyCode::KeyF10),
            "Key_F11" => Some(KeyCode::KeyF11),
            "Key_F12" => Some(KeyCode::KeyF12),
            "Key_Escape" => Some(KeyCode::KeyEscape),
            "Key_Tab" => Some(KeyCode::KeyTab),
            "Key_Enter" => Some(KeyCode::KeyEnter),
            "Key_Menu" => Some(KeyCode::KeyMenu),
            "Key_Insert" => Some(KeyCode::KeyInsert),
            "Key_Pause" => Some(KeyCode::KeyPause),
            "Key_Space" => Some(KeyCode::KeySpace),
            "Key_Home" => Some(KeyCode::KeyHome),
            "Key_End" => Some(KeyCode::KeyEnd),
            "Key_Delete" => Some(KeyCode::KeyDelete),
            "Key_Page_Down" => Some(KeyCode::KeyPageDown),
            "Key_Page_Up" => Some(KeyCode::KeyPageUp),
            "Key_Caps_Lock" => Some(KeyCode::KeyCapsLock),
            "Key_Backspace" => Some(KeyCode::KeyBackspace),
            "Key_Print_Screen" => Some(KeyCode::KeyPrintScreen),
            "Key_Num_1" => Some(KeyCode::KeyNum1),
            "Key_Num_2" => Some(KeyCode::KeyNum2),
            "Key_Num_3" => Some(KeyCode::KeyNum3),
            "Key_Num_4" => Some(KeyCode::KeyNum4),
            "Key_Num_5" => Some(KeyCode::KeyNum5),
            "Key_Num_6" => Some(KeyCode::KeyNum6),
            "Key_Num_7" => Some(KeyCode::KeyNum7),
            "Key_Num_8" => Some(KeyCode::KeyNum8),
            "Key_Num_9" => Some(KeyCode::KeyNum9),
            "Key_Num_0" => Some(KeyCode::KeyNum0),
            "Key_Num_Add" => Some(KeyCode::KeyNumAdd),
            "Key_Num_Substract" => Some(KeyCode::KeyNumSubstract),
            "Key_Num_Multiply" => Some(KeyCode::KeyNumMultiply),
            "Key_Num_Divide" => Some(KeyCode::KeyNumDivide),
            "Key_Num_Decimal_Point" => Some(KeyCode::KeyNumDecimalPoint),
            "Key_Num_Lock" => Some(KeyCode::KeyNumLock),
            "Key_Num_Enter" => Some(KeyCode::KeyNumEnter),
            "Key_Fn" => Some(KeyCode::KeyFn),
            "Shortcut_Explorer" => Some(KeyCode::ShortcutExplorer),
            "Shortcut_Cut" => Some(KeyCode::ShortcutCut),
            "Shortcut_Copy" => Some(KeyCode::ShortcutCopy),
            "Shortcut_Paste" => Some(KeyCode::ShortcutPaste),
            "Shortcut_Save" => Some(KeyCode::ShortcutSave),
            "Shortcut_Show_Desktop" => Some(KeyCode::ShortcutShowDesktop),
            "Shortcut_Lock" => Some(KeyCode::ShortcutLock),
            "Shortcut_Switch_Window" => Some(KeyCode::ShortcutSwitchWindow),
            "Shortcut_Close_Window" => Some(KeyCode::ShortcutCloseWindow),
            "Multimedia_Play_Pause" => Some(KeyCode::MultimediaPlayPause),
            "Multimedia_Stop" => Some(KeyCode::MultimediaStop),
            "Multimedia_Previous" => Some(KeyCode::MultimediaPrevious),
            "Multimedia_Next" => Some(KeyCode::MultimediaNext),
            "Multimedia_Volume_Up" => Some(KeyCode::MultimediaVolumeUp),
            "Multimedia_Volume_Down" => Some(KeyCode::MultimediaVolumeDown),
            "Multimedia_Mute" => Some(KeyCode::MultimediaMute),
            _ => None,
        }
    }

    pub fn to_u32(self) -> u32 {
        self as u32
    }

    /// Convert u32 to KeyCode
    /// Since KeyCode uses #[repr(u32)], we can safely transmute any u32 value
    pub fn from_u32(value: u32) -> KeyCode {
        // This is safe because we're using #[repr(u32)] which guarantees
        // the enum has the same memory layout as u32
        unsafe { std::mem::transmute(value) }
    }
}

impl Serialize for KeyCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.to_u32())
    }
}

impl<'de> Deserialize<'de> for KeyCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, Visitor};
        use std::fmt;

        struct KeyCodeVisitor;

        impl<'de> Visitor<'de> for KeyCodeVisitor {
            type Value = KeyCode;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a u32 key code or a string enum variant name")
            }

            fn visit_u32<E>(self, value: u32) -> Result<KeyCode, E>
            where
                E: de::Error,
            {
                Ok(KeyCode::from_u32(value))
            }

            fn visit_u64<E>(self, value: u64) -> Result<KeyCode, E>
            where
                E: de::Error,
            {
                if value > u32::MAX as u64 {
                    return Err(E::custom(format!("key code value too large: {}", value)));
                }
                Ok(KeyCode::from_u32(value as u32))
            }

            fn visit_i64<E>(self, value: i64) -> Result<KeyCode, E>
            where
                E: de::Error,
            {
                if value < 0 || value > u32::MAX as i64 {
                    return Err(E::custom(format!("key code value out of range: {}", value)));
                }
                Ok(KeyCode::from_u32(value as u32))
            }

            fn visit_str<E>(self, value: &str) -> Result<KeyCode, E>
            where
                E: de::Error,
            {
                KeyCode::from_str(value)
                    .ok_or_else(|| E::custom(format!("unknown key code variant: {}", value)))
            }
        }

        deserializer.deserialize_any(KeyCodeVisitor)
    }
}

/// Represents a single key on the keyboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Key {
    pub buffer_index: u8,
    pub key_code: KeyCode,
    pub top_x: i32,
    pub top_y: i32,
    pub bottom_x: i32,
    pub bottom_y: i32,
}

/// Keyboard configuration and capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keyboard {
    pub id: KeyboardUsbId,
    pub path: String,
    pub name: String,
    pub image_path: String,
    pub keys: Vec<Key>,
    pub key_map_enabled: bool,
    pub light_enabled: bool,
    pub rgb: bool,
    pub top_left_x: i32,
    pub top_left_y: i32,
    pub bottom_right_x: i32,
    pub bottom_right_y: i32,
}

/// Lighting mode with mode bit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mode {
    pub name: String,
    pub mode_bit: u8,
}

/// RGB color
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// Keyboard configuration to send
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardConfig {
    pub light_mode: Option<LightModeConfig>,
    pub key_mapping: Option<KeyMappingConfig>,
}

/// Light mode configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightModeConfig {
    pub mode_bit: u8,
    pub animation: u8,
    pub brightness: u8,
    pub color: Option<RgbColor>,
    pub random_colors: bool,
    pub sleep: u8,
    pub custom_colors: Option<Vec<PerKeyColor>>,
}

/// Per-key color for custom light mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerKeyColor {
    pub buffer_index: u8,
    pub color: RgbColor,
}

/// Key mapping configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMappingConfig {
    pub mappings: Vec<KeyMapping>,
}

/// Single key mapping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMapping {
    pub buffer_index: u8,
    pub key_code: KeyCode,
}

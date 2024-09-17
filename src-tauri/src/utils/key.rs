use windows::Win32::UI::Input::KeyboardAndMouse::*;

pub struct KeyChord {
    keys: Vec<Key>,
}

impl KeyChord {
    pub fn new(keys: Vec<Key>) -> Self {
        KeyChord { keys }
    }

    pub fn from_string(string: &str) -> Self {
        let keys = string.split("+").map(|key| Key::from_name(key.trim())).collect();
        KeyChord::new(keys)
    }

    pub fn to_string(&self) -> String {
        self.keys.iter().map(|key| key.name.clone()).collect::<Vec<String>>().join(" + ")
    }

    pub fn is_pressed(&self, pressed_keys: &Vec<u32>) -> bool {
        for key in self.keys.iter() {
            if !pressed_keys.contains(&(key.vk_code.0 as u32)) {
                return false;
            }
        }
        true
    }
}

pub struct Key {
    name: String,
    vk_code: VIRTUAL_KEY,
}

impl Key {
    pub fn new(name: String, vk_code: VIRTUAL_KEY) -> Self {
        Key { name, vk_code }
    }

    pub fn from_name(name: &str) -> Self {
        let vk_code = to_vk_code(name).expect(format!("Unknown key: {}", name).as_str());
        Key::new(name.to_string(), vk_code)
    }
}

pub fn to_vk_code(name: &str) -> Option<VIRTUAL_KEY> {
    match name.to_uppercase().as_str() {
        "CTRL" => Some(VK_CONTROL),
        "LCTRL" => Some(VK_LCONTROL),
        "RCTRL" => Some(VK_RCONTROL),

        "SHIFT" => Some(VK_SHIFT),
        "LSHIFT" => Some(VK_LSHIFT),
        "RSHIFT" => Some(VK_RSHIFT),

        "ALT" => Some(VK_MENU),
        "LALT" => Some(VK_LMENU),
        "RALT" => Some(VK_RMENU),

        "A" => Some(VK_A),
        "B" => Some(VK_B),
        "C" => Some(VK_C),
        "D" => Some(VK_D),
        "E" => Some(VK_E),
        "F" => Some(VK_F),
        "G" => Some(VK_G),
        "H" => Some(VK_H),
        "I" => Some(VK_I),
        "J" => Some(VK_J),
        "K" => Some(VK_K),
        "L" => Some(VK_L),
        "M" => Some(VK_M),
        "N" => Some(VK_N),
        "O" => Some(VK_O),
        "P" => Some(VK_P),
        "Q" => Some(VK_Q),
        "R" => Some(VK_R),
        "S" => Some(VK_S),
        "T" => Some(VK_T),
        "U" => Some(VK_U),
        "V" => Some(VK_V),
        "W" => Some(VK_W),
        "X" => Some(VK_X),
        "Y" => Some(VK_Y),
        "Z" => Some(VK_Z),

        "0" => Some(VK_0),
        "1" => Some(VK_1),
        "2" => Some(VK_2),
        "3" => Some(VK_3),
        "4" => Some(VK_4),
        "5" => Some(VK_5),
        "6" => Some(VK_6),
        "7" => Some(VK_7),
        "8" => Some(VK_8),
        "9" => Some(VK_9),

        "F1" => Some(VK_F1),
        "F2" => Some(VK_F2),
        "F3" => Some(VK_F3),
        "F4" => Some(VK_F4),
        "F5" => Some(VK_F5),
        "F6" => Some(VK_F6),
        "F7" => Some(VK_F7),
        "F8" => Some(VK_F8),
        "F9" => Some(VK_F9),
        "F10" => Some(VK_F10),
        "F11" => Some(VK_F11),
        "F12" => Some(VK_F12),

        "NUM0" => Some(VK_NUMPAD0),
        "NUM1" => Some(VK_NUMPAD1),
        "NUM2" => Some(VK_NUMPAD2),
        "NUM3" => Some(VK_NUMPAD3),
        "NUM4" => Some(VK_NUMPAD4),
        "NUM5" => Some(VK_NUMPAD5),
        "NUM6" => Some(VK_NUMPAD6),
        "NUM7" => Some(VK_NUMPAD7),
        "NUM8" => Some(VK_NUMPAD8),
        "NUM9" => Some(VK_NUMPAD9),

        "NUMLOCK" => Some(VK_NUMLOCK),
        "NUMSLASH" => Some(VK_DIVIDE),
        "NUMMULTIPLY" => Some(VK_MULTIPLY),
        "NUMMINUS" => Some(VK_SUBTRACT),
        "NUMPLUS" => Some(VK_ADD),
        "NUMENTER" => Some(VK_RETURN),
        "NUMDECIMAL" => Some(VK_DECIMAL),

        "BACK" => Some(VK_BACK),
        "TAB" => Some(VK_TAB),
        "ENTER" => Some(VK_RETURN),
        "SPACE" => Some(VK_SPACE),
        "CAPSLOCK" => Some(VK_CAPITAL),
        "ESC" => Some(VK_ESCAPE),

        "LEFT" => Some(VK_LEFT),
        "RIGHT" => Some(VK_RIGHT),
        "UP" => Some(VK_UP),
        "DOWN" => Some(VK_DOWN),

        "HOME" => Some(VK_HOME),
        "END" => Some(VK_END),
        "PAGEUP" => Some(VK_PRIOR),
        "PAGEDOWN" => Some(VK_NEXT),

        "INSERT" => Some(VK_INSERT),
        "DELETE" => Some(VK_DELETE),

        "PRINTSCREEN" => Some(VK_SNAPSHOT),
        "SCROLLLOCK" => Some(VK_SCROLL),

        "PAUSE" => Some(VK_PAUSE),
        "BREAK" => Some(VK_CANCEL),
        "MENU" => Some(VK_MENU),
        "LMENU" => Some(VK_LMENU),
        "RMENU" => Some(VK_RMENU),

        "LWIN" => Some(VK_LWIN),
        "RWIN" => Some(VK_RWIN),

        "APPS" => Some(VK_APPS),
        "SLEEP" => Some(VK_SLEEP),
        "ZOOM" => Some(VK_ZOOM),

        "VOLUMEUP" => Some(VK_VOLUME_UP),
        "VOLUMEDOWN" => Some(VK_VOLUME_DOWN),
        "VOLUMEMUTE" => Some(VK_VOLUME_MUTE),

        "STOP" => Some(VK_MEDIA_STOP),
        "PLAYPAUSE" => Some(VK_MEDIA_PLAY_PAUSE),
        "PREV" => Some(VK_MEDIA_PREV_TRACK),
        "NEXT" => Some(VK_MEDIA_NEXT_TRACK),

        _ => None,
    }
}

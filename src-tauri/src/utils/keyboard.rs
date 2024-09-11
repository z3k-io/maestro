use std::collections::HashSet;

use inputbot::KeybdKey;

pub fn parse_key_chord(chord: &str) -> HashSet<KeybdKey> {
    chord
        .split('+')
        .filter_map(|key| match key.trim().to_lowercase().as_str() {
            "ctrl" => Some(KeybdKey::LControlKey),
            "shift" => Some(KeybdKey::LShiftKey),
            "alt" => Some(KeybdKey::LAltKey),
            "up" => Some(KeybdKey::UpKey),
            "down" => Some(KeybdKey::DownKey),
            "left" => Some(KeybdKey::LeftKey),
            "right" => Some(KeybdKey::RightKey),
            "home" => Some(KeybdKey::HomeKey),
            "end" => Some(KeybdKey::EndKey),
            "pageup" => Some(KeybdKey::PageUpKey),
            "pagedown" => Some(KeybdKey::PageDownKey),
            "backspace" => Some(KeybdKey::BackspaceKey),
            "tab" => Some(KeybdKey::TabKey),
            "enter" => Some(KeybdKey::EnterKey),
            "escape" => Some(KeybdKey::EscapeKey),
            "space" => Some(KeybdKey::SpaceKey),
            "delete" => Some(KeybdKey::DeleteKey),
            "insert" => Some(KeybdKey::InsertKey),
            "num0" => Some(KeybdKey::Numrow0Key),
            "num1" => Some(KeybdKey::Numrow1Key),
            "num2" => Some(KeybdKey::Numrow2Key),
            "num3" => Some(KeybdKey::Numrow3Key),
            "num4" => Some(KeybdKey::Numrow4Key),
            "num5" => Some(KeybdKey::Numrow5Key),
            "num6" => Some(KeybdKey::Numrow6Key),
            "num7" => Some(KeybdKey::Numrow7Key),
            "num8" => Some(KeybdKey::Numrow8Key),
            "num9" => Some(KeybdKey::Numrow9Key),
            "a" => Some(KeybdKey::AKey),
            "b" => Some(KeybdKey::BKey),
            "c" => Some(KeybdKey::CKey),
            "d" => Some(KeybdKey::DKey),
            "e" => Some(KeybdKey::EKey),
            "f" => Some(KeybdKey::FKey),
            "g" => Some(KeybdKey::GKey),
            "h" => Some(KeybdKey::HKey),
            "i" => Some(KeybdKey::IKey),
            "j" => Some(KeybdKey::JKey),
            "k" => Some(KeybdKey::KKey),
            "l" => Some(KeybdKey::LKey),
            "m" => Some(KeybdKey::MKey),
            "n" => Some(KeybdKey::NKey),
            "o" => Some(KeybdKey::OKey),
            "p" => Some(KeybdKey::PKey),
            "q" => Some(KeybdKey::QKey),
            "r" => Some(KeybdKey::RKey),
            "s" => Some(KeybdKey::SKey),
            "t" => Some(KeybdKey::TKey),
            "u" => Some(KeybdKey::UKey),
            "v" => Some(KeybdKey::VKey),
            "w" => Some(KeybdKey::WKey),
            "x" => Some(KeybdKey::XKey),
            "y" => Some(KeybdKey::YKey),
            "z" => Some(KeybdKey::ZKey),
            "f1" => Some(KeybdKey::F1Key),
            "f2" => Some(KeybdKey::F2Key),
            "f3" => Some(KeybdKey::F3Key),
            "f4" => Some(KeybdKey::F4Key),
            "f5" => Some(KeybdKey::F5Key),
            "f6" => Some(KeybdKey::F6Key),
            "f7" => Some(KeybdKey::F7Key),
            "f8" => Some(KeybdKey::F8Key),
            "f9" => Some(KeybdKey::F9Key),
            "f10" => Some(KeybdKey::F10Key),
            "f11" => Some(KeybdKey::F11Key),
            "f12" => Some(KeybdKey::F12Key),
            "f13" => Some(KeybdKey::F13Key),
            "f14" => Some(KeybdKey::F14Key),
            "f15" => Some(KeybdKey::F15Key),
            "f16" => Some(KeybdKey::F16Key),
            "f17" => Some(KeybdKey::F17Key),
            "f18" => Some(KeybdKey::F18Key),
            "f19" => Some(KeybdKey::F19Key),
            "f20" => Some(KeybdKey::F20Key),
            "f21" => Some(KeybdKey::F21Key),
            "f22" => Some(KeybdKey::F22Key),
            "f23" => Some(KeybdKey::F23Key),
            "f24" => Some(KeybdKey::F24Key),
            "numlock" => Some(KeybdKey::NumLockKey),
            "capslock" => Some(KeybdKey::CapsLockKey),
            "scrolllock" => Some(KeybdKey::ScrollLockKey),
            "semicolon" => Some(KeybdKey::SemicolonKey),
            "equal" => Some(KeybdKey::EqualKey),
            "minus" => Some(KeybdKey::MinusKey),
            "period" => Some(KeybdKey::PeriodKey),
            "comma" => Some(KeybdKey::CommaKey),
            "slash" => Some(KeybdKey::SlashKey),
            "backtick" => Some(KeybdKey::BackquoteKey),
            "quote" => Some(KeybdKey::QuoteKey),
            "backslash" => Some(KeybdKey::BackslashKey),
            "lbracket" => Some(KeybdKey::LBracketKey),
            "rbracket" => Some(KeybdKey::RBracketKey),
            "volumemute" => Some(KeybdKey::VolumeMuteKey),
            "volumedown" => Some(KeybdKey::VolumeDownKey),
            "volumeup" => Some(KeybdKey::VolumeUpKey),

            // Add more mappings as needed
            _ => {
                log::warn!("Unsupported key in chord: {}", key);
                None
            }
        })
        .collect()
}

pub fn is_chord_pressed(chord: &HashSet<KeybdKey>) -> bool {
    let result = chord.iter().all(|&k| k.is_pressed());
    log::debug!("Chord {:?} pressed: {}", chord, result);
    return result;
}

// pub fn is_modifier_key(key: KeybdKey) -> bool {
//     matches!(
//         key,
//         KeybdKey::LControlKey | KeybdKey::RControlKey | KeybdKey::LShiftKey | KeybdKey::RShiftKey | KeybdKey::LAltKey | KeybdKey::RAltKey
//     )
// }

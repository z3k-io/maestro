use inputbot::{KeybdKey::*, *};
use std::{collections::HashSet, sync::Arc, thread};
use tauri::Window;

use crate::{config::get_config, events::AppEvent, volume_manager};

#[derive(Clone)]
struct WindowWrapper(Arc<Window>);

impl WindowWrapper {
    fn new(window: Window) -> Self {
        WindowWrapper(Arc::new(window))
    }

    fn show_and_emit(&self, event: AppEvent, payload: String) {
        self.0.show().unwrap();
        self.0.emit(event.as_str(), payload).unwrap();
    }
}

fn parse_key_chord(chord: &str) -> HashSet<KeybdKey> {
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

            // Add more mappings as needed
            _ => {
                log::warn!("Unsupported key in chord: {}", key);
                None
            }
        })
        .collect()
}

fn is_chord_pressed(chord: &HashSet<KeybdKey>) -> bool {
    let result = chord.iter().all(|&k| k.is_pressed());
    log::debug!("Chord {:?} pressed: {}", chord, result);
    result
}

fn is_modifier_key(key: KeybdKey) -> bool {
    matches!(
        key,
        KeybdKey::LControlKey | KeybdKey::RControlKey | KeybdKey::LShiftKey | KeybdKey::RShiftKey | KeybdKey::LAltKey | KeybdKey::RAltKey
    )
}

pub fn override_media_keys(window: Window) {
    log::info!("Initializing key listeners");

    let window = WindowWrapper::new(window);
    let config = get_config();

    // Parse all key chords from config once
    let parsed_keybinds: Vec<(HashSet<KeybdKey>, String, String)> = config
        .sessions
        .iter()
        .flat_map(|session| {
            session.keybinds.iter().flat_map(|keybinds| {
                keybinds.iter().filter_map(|keybind| {
                    let chord = parse_key_chord(&keybind.key);
                    if !chord.is_empty() {
                        Some((chord, keybind.action.clone(), session.name.clone()))
                    } else {
                        None
                    }
                })
            })
        })
        .collect();

    // Custom key bindings from config
    KeybdKey::bind_all({
        let window = window.clone();
        let parsed_keybinds = parsed_keybinds.clone();
        move |event| {
            if !is_modifier_key(event) {
                for (chord, action, session_name) in &parsed_keybinds {
                    if chord.contains(&event) && is_chord_pressed(chord) {
                        log::info!("Matched key chord: {:?}", chord);
                        handle_action(action, session_name, window.clone());
                    }
                }
            }
        }
    });

    // Hard-coded media key overrides
    VolumeUpKey.block_bind({
        let window = window.clone();
        move || {
            log::debug!("[MEDIA KEY] Volume Up");
            handle_session_up("master", window.clone());
        }
    });

    VolumeDownKey.block_bind({
        let window = window.clone();
        move || {
            log::debug!("[MEDIA KEY] Volume down");
            handle_session_down("master", window.clone());
        }
    });

    VolumeMuteKey.block_bind({
        let window = window.clone();
        move || {
            log::debug!("[MEDIA KEY] Mute");
            handle_session_toggle_mute("master", window.clone());
        }
    });

    thread::spawn(move || {
        inputbot::handle_input_events();
    });
}

fn handle_session_up(session_name: &str, window: WindowWrapper) {
    log::info!("Session up: {}", session_name);
    let current_vol = volume_manager::get_session_volume(session_name);
    let updated_vol = volume_manager::set_session_volume(session_name, current_vol + 2);

    let payload = format!("{}:{}", session_name, updated_vol);
    window.show_and_emit(AppEvent::VolumeChange, payload);
}

fn handle_session_down(session_name: &str, window: WindowWrapper) {
    log::info!("Session down: {}", session_name);
    let current_vol = volume_manager::get_session_volume(session_name);
    let updated_vol = volume_manager::set_session_volume(session_name, current_vol - 2);

    let payload = format!("{}:{}", session_name, updated_vol);
    window.show_and_emit(AppEvent::VolumeChange, payload);
}

fn handle_session_toggle_mute(session_name: &str, window: WindowWrapper) {
    log::info!("Session toggle mute: {}", session_name);
    let mute = volume_manager::toggle_session_mute(session_name);
    let mut volume = volume_manager::get_session_volume(session_name);

    if mute {
        volume = volume * -1
    }

    let payload = format!("{}:{}", session_name, volume);
    window.show_and_emit(AppEvent::VolumeChange, payload);
}

fn handle_action(action: &str, session_name: &str, window: WindowWrapper) {
    match action {
        "VolumeUp" => handle_session_up(session_name, window),
        "VolumeDown" => handle_session_down(session_name, window),
        "ToggleMute" => handle_session_toggle_mute(session_name, window),
        _ => log::warn!("Unknown action: {}", action),
    }
}

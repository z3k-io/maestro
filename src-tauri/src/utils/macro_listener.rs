use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};

use inputbot::KeybdKey;
use tauri::AppHandle;
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut, ShortcutState};

use crate::api::events;
use crate::{
    config,
    services::{volume_service, window_service},
};
use inputbot::KeybdKey::{VolumeDownKey, VolumeMuteKey, VolumeUpKey};

use super::keyboard;

struct HotkeyState {
    volume_up_active: Arc<AtomicBool>,
    volume_down_active: Arc<AtomicBool>,
    last_volume_up_press: Arc<Mutex<Instant>>,
    last_volume_down_press: Arc<Mutex<Instant>>,
    last_volume_action: Arc<Mutex<Instant>>,
}
impl HotkeyState {
    fn new() -> Self {
        HotkeyState {
            volume_up_active: Arc::new(AtomicBool::new(false)),
            volume_down_active: Arc::new(AtomicBool::new(false)),
            last_volume_up_press: Arc::new(Mutex::new(Instant::now())),
            last_volume_down_press: Arc::new(Mutex::new(Instant::now())),
            last_volume_action: Arc::new(Mutex::new(Instant::now())),
        }
    }
}

pub fn initialize_key_listeners(app_handle: AppHandle) {
    // Subscribe with global-hotkey (to work when window is focused)

    let mute_shortcut = Shortcut::new(Some(Modifiers::empty()), Code::AudioVolumeMute);
    let volume_up_shortcut = Shortcut::new(Some(Modifiers::empty()), Code::AudioVolumeUp);
    let volume_down_shortcut = Shortcut::new(Some(Modifiers::empty()), Code::AudioVolumeDown);
    let mut mixer_shortcut: Option<Shortcut> = None;

    let mut shortcuts = vec![mute_shortcut, volume_up_shortcut, volume_down_shortcut];

    let mixer_hotkey = config::get_config().mixer.hotkey.clone();

    if let Some(mixer_hotkey) = mixer_hotkey.clone() {
        mixer_shortcut = Some(Shortcut::try_from(mixer_hotkey).expect("Failed to parse hotkey"));
        shortcuts.push(mixer_shortcut.unwrap());
    }

    let hotkey_state = Arc::new(HotkeyState::new());
    let hotkey_state_clone = hotkey_state.clone();

    let _ = app_handle.plugin(tauri_plugin_global_shortcut::Builder::new().build());
    // .with_shortcuts(shortcuts)
    // .expect("Failed to initialize shortcuts")
    // .with_handler(move |app_handle, shortcut, event| {
    // log::warn!("Shortcut pressed: {}", shortcut.into_string());
    // if shortcut.id() == mixer_shortcut.unwrap().id() && event.state() == ShortcutState::Pressed {
    //     log::info!("Mixer macro pressed: {}", shortcut.into_string());
    //     window_service::toggle_mixer(app_handle.clone());
    //     return;
    // }

    // if shortcut.id() == mute_shortcut.id() && event.state() == ShortcutState::Pressed {
    //     log::info!("Mute pressed");
    //     handle_session_toggle_mute("master", app_handle.clone());
    //     return;
    // }

    // if shortcut.id() == volume_up_shortcut.id() {
    //     if event.state() == ShortcutState::Pressed {
    //         log::info!("VolumeUp Pressed");
    //         hotkey_state.volume_up_active.store(true, Ordering::SeqCst);
    //         let window_clone = app_handle.clone();

    //         let state_clone = hotkey_state.clone();
    //         std::thread::spawn(move || {
    //             // Execute immediately for the first press
    //             handle_session_up("master", window_clone.clone());

    //             thread::sleep(Duration::from_millis(250));

    //             // Incrementally execute every 25ms, following the 250ms delay
    //             while state_clone.volume_up_active.load(Ordering::SeqCst) {
    //                 let elapsed = state_clone.last_volume_up_press.lock().unwrap().elapsed();
    //                 if elapsed >= Duration::from_millis(250) {
    //                     handle_session_up("master", window_clone.clone());
    //                 }
    //                 std::thread::sleep(Duration::from_millis(25));
    //             }
    //         });
    //     }
    //     if event.state() == ShortcutState::Released {
    //         log::info!("VolumeUp Media Key Released");
    //         hotkey_state.volume_up_active.store(false, Ordering::SeqCst);
    //     }
    //     return;
    // }

    // if shortcut.id() == volume_down_shortcut.id() {
    //     if event.state() == ShortcutState::Pressed {
    //         log::info!("VolumeDown Pressed");
    //         hotkey_state.volume_down_active.store(true, Ordering::SeqCst);
    //         let window_clone = app_handle.clone();

    //         let state_clone = hotkey_state.clone();
    //         std::thread::spawn(move || {
    //             // Execute immediately for the first press
    //             handle_session_down("master", window_clone.clone());

    //             thread::sleep(Duration::from_millis(250));

    //             // Incrementally execute every 25ms, following the 250ms delay
    //             while state_clone.volume_down_active.load(Ordering::SeqCst) {
    //                 let elapsed = state_clone.last_volume_down_press.lock().unwrap().elapsed();
    //                 if elapsed >= Duration::from_millis(250) {
    //                     handle_session_down("master", window_clone.clone());
    //                 }
    //                 std::thread::sleep(Duration::from_millis(25));
    //             }
    //         });
    //     }
    //     if event.state() == ShortcutState::Released {
    //         log::info!("VolumeDown Media Key Released");
    //         hotkey_state.volume_down_active.store(false, Ordering::SeqCst);
    //     }
    // }
    // return;
    // })
    // .build(),
    // );

    // Subscribe with inputbot (to work when an app is fullscreen (borderless))

    if let Some(mixer_hotkey) = mixer_hotkey.clone() {
        let mixer_chord = keyboard::parse_key_chord(&mixer_hotkey);
        let app_handle = app_handle.clone();
        KeybdKey::bind_all({
            move |event| {
                if mixer_chord.contains(&event) && keyboard::is_chord_pressed(&mixer_chord) {
                    log::info!("Ctrl+Shift+M combo pressed");
                    window_service::toggle_mixer(app_handle.clone());
                }
            }
        });
    }

    let app_handle_clone = app_handle.clone();
    let state_clone = hotkey_state_clone.clone();
    VolumeUpKey.block_bind(move || {
        if should_handle_action(&state_clone.last_volume_action) {
            log::warn!("[MEDIA KEY] Volume Up");
            handle_session_up("master", app_handle_clone.clone());
        }
    });

    let app_handle_clone = app_handle.clone();
    let state_clone = hotkey_state_clone.clone();
    VolumeDownKey.block_bind(move || {
        if should_handle_action(&state_clone.last_volume_action) {
            log::warn!("[MEDIA KEY] Volume down");
            handle_session_down("master", app_handle_clone.clone());
        }
    });

    let app_handle_clone = app_handle.clone();
    let state_clone = hotkey_state_clone.clone();
    VolumeMuteKey.block_bind(move || {
        if should_handle_action(&state_clone.last_volume_action) {
            log::warn!("[MEDIA KEY] Mute");
            handle_session_toggle_mute("master", app_handle_clone.clone());
        }
    });

    thread::spawn(|| {
        inputbot::handle_input_events();
    });
}

fn should_handle_action(last_action: &Arc<Mutex<Instant>>) -> bool {
    let mut last = last_action.lock().unwrap();
    let now = Instant::now();
    if now.duration_since(*last) > Duration::from_millis(100) {
        *last = now;
        true
    } else {
        false
    }
}

fn handle_session_toggle_mute(session_name: &str, app_handle: AppHandle) {
    log::info!("Session toggle mute: {}", session_name);
    let session = volume_service::toggle_session_mute(session_name);

    events::emit_volume_change_event(&session, app_handle);
}

fn handle_session_up(session_name: &str, app_handle: AppHandle) {
    log::info!("Session up: {}", session_name);
    let current_vol = volume_service::get_session_volume(session_name);
    let session = volume_service::set_session_volume(session_name, current_vol + 2).unwrap();

    events::emit_volume_change_event(&session, app_handle);
}

fn handle_session_down(session_name: &str, app_handle: AppHandle) {
    log::info!("Session down: {}", session_name);
    let current_vol = volume_service::get_session_volume(session_name);
    let session = volume_service::set_session_volume(session_name, current_vol - 2).unwrap();

    events::emit_volume_change_event(&session, app_handle);
}

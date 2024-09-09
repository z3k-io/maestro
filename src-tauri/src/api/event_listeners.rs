use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, HotKeyState,
};

use std::sync::atomic::{AtomicBool, Ordering};
use std::{
    sync::{mpsc::Sender, Arc, Mutex},
    thread,
    time::{Duration, Instant},
};
use tauri::{AppHandle, GlobalShortcutManager, Manager, Window};

use crate::{
    api::{self, events::AppEvent},
    config::get_config,
    models::audio_session::AudioSession,
    utils::keyboard,
    volume_manager, window_manager,
};
use inputbot::KeybdKey::{self, VolumeDownKey, VolumeMuteKey, VolumeUpKey};

#[derive(Clone)]
struct WindowWrapper(Arc<Window>);

impl WindowWrapper {
    fn new(window: Window) -> Self {
        WindowWrapper(Arc::new(window))
    }

    fn show_and_emit(&self, event: AppEvent, session: AudioSession) {
        self.0.emit(event.as_str(), &session).unwrap();
        self.0.show().unwrap();
    }
}

struct HotkeyState {
    volume_up_active: Arc<AtomicBool>,
    volume_down_active: Arc<AtomicBool>,
    last_volume_up_press: Arc<Mutex<Instant>>,
    last_volume_down_press: Arc<Mutex<Instant>>,
}
impl HotkeyState {
    fn new() -> Self {
        HotkeyState {
            volume_up_active: Arc::new(AtomicBool::new(false)),
            volume_down_active: Arc::new(AtomicBool::new(false)),
            last_volume_up_press: Arc::new(Mutex::new(Instant::now())),
            last_volume_down_press: Arc::new(Mutex::new(Instant::now())),
        }
    }
}

// TODO: We are currently doublely listening to media keys, the global hotkey library supports key press and release
// which we need to handle a press and hold correctly, however it doesnt work for full screened apps. We also use inputbot
// which works for full screened apps, but doesn't support key release events. Hence the redundancy for now...

pub fn initialize(tx: Sender<HotKey>, window: Window, app: AppHandle) {
    let window = WindowWrapper::new(window);
    register_media_key_listeners(tx, window.clone());
    register_mixer_hotkey(&app);
    override_media_keys(window.clone(), app.clone());
}

fn register_media_key_listeners(tx: Sender<HotKey>, window: WindowWrapper) {
    log::warn!("Registering hotkeys");

    // let window = WindowWrapper::new(window);
    let hotkey_state = Arc::new(HotkeyState::new());

    // Need to register hotkeys with all the different modifier combinations
    // TODO: Is there a smarter way to do this? Currently multiple modifiers aren't captured
    let volume_up = HotKey::new(None, Code::AudioVolumeUp);
    let volume_up2 = HotKey::new(Some(Modifiers::CONTROL), Code::AudioVolumeUp);
    let volume_up4 = HotKey::new(Some(Modifiers::ALT), Code::AudioVolumeUp);
    let volume_up3 = HotKey::new(Some(Modifiers::SHIFT), Code::AudioVolumeUp);
    let volume_up5 = HotKey::new(Some(Modifiers::SUPER), Code::AudioVolumeUp);
    tx.send(volume_up).unwrap();
    tx.send(volume_up2).unwrap();
    tx.send(volume_up4).unwrap();
    tx.send(volume_up3).unwrap();
    tx.send(volume_up5).unwrap();

    let volume_down = HotKey::new(None, Code::AudioVolumeDown);
    let volume_down2 = HotKey::new(Some(Modifiers::CONTROL), Code::AudioVolumeDown);
    let volume_down4 = HotKey::new(Some(Modifiers::ALT), Code::AudioVolumeDown);
    let volume_down3 = HotKey::new(Some(Modifiers::SHIFT), Code::AudioVolumeDown);
    let volume_down5 = HotKey::new(Some(Modifiers::SUPER), Code::AudioVolumeDown);
    tx.send(volume_down).unwrap();
    tx.send(volume_down2).unwrap();
    tx.send(volume_down4).unwrap();
    tx.send(volume_down3).unwrap();
    tx.send(volume_down5).unwrap();

    let mute = HotKey::new(None, Code::AudioVolumeMute);
    let mute2 = HotKey::new(Some(Modifiers::CONTROL), Code::AudioVolumeMute);
    let mute4 = HotKey::new(Some(Modifiers::ALT), Code::AudioVolumeMute);
    let mute3 = HotKey::new(Some(Modifiers::SHIFT), Code::AudioVolumeMute);
    let mute5 = HotKey::new(Some(Modifiers::SUPER), Code::AudioVolumeMute);
    tx.send(mute).unwrap();
    tx.send(mute2).unwrap();
    tx.send(mute4).unwrap();
    tx.send(mute3).unwrap();
    tx.send(mute5).unwrap();

    let hotkey_state_clone = hotkey_state.clone();
    thread::spawn(move || loop {
        if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
            if event.id() == volume_up.id()
                || event.id() == volume_up2.id()
                || event.id() == volume_up4.id()
                || event.id() == volume_up3.id()
                || event.id() == volume_up5.id()
            {
                if event.state() == HotKeyState::Pressed {
                    log::info!("Volume Up Media Key Pressed");
                    hotkey_state_clone.volume_up_active.store(true, Ordering::SeqCst);
                    let window_clone = window.clone();
                    let state_clone = hotkey_state_clone.clone();

                    std::thread::spawn(move || {
                        // Execute immediately for the first press
                        handle_session_up("master", window_clone.clone());

                        // Wait for 500ms before starting repeated execution
                        std::thread::sleep(Duration::from_millis(500));

                        // Incrementally execute every 25ms
                        while state_clone.volume_up_active.load(Ordering::SeqCst) {
                            let elapsed = state_clone.last_volume_up_press.lock().unwrap().elapsed();
                            if elapsed >= Duration::from_millis(500) {
                                handle_session_up("master", window_clone.clone());
                            }
                            std::thread::sleep(Duration::from_millis(25));
                        }
                    });
                } else if event.state() == HotKeyState::Released {
                    log::info!("Volume Up Media Key Released");
                    hotkey_state_clone.volume_up_active.store(false, Ordering::SeqCst);
                }
            } else if event.id() == volume_down.id()
                || event.id() == volume_down2.id()
                || event.id() == volume_down4.id()
                || event.id() == volume_down3.id()
                || event.id() == volume_down5.id()
            {
                if event.state() == HotKeyState::Pressed {
                    log::info!("Volume Down Media Key Pressed");
                    hotkey_state_clone.volume_down_active.store(true, Ordering::SeqCst);
                    let window_clone = window.clone();
                    let state_clone = hotkey_state_clone.clone();

                    std::thread::spawn(move || {
                        // Execute immediately for the first press
                        handle_session_down("master", window_clone.clone());

                        // Wait for 500ms before starting repeated execution
                        std::thread::sleep(Duration::from_millis(500));

                        // Incrementally execute every 25ms
                        while state_clone.volume_down_active.load(Ordering::SeqCst) {
                            let elapsed = state_clone.last_volume_down_press.lock().unwrap().elapsed();
                            if elapsed >= Duration::from_millis(500) {
                                handle_session_down("master", window_clone.clone());
                            }
                            std::thread::sleep(Duration::from_millis(25));
                        }
                    });
                } else if event.state() == HotKeyState::Released {
                    log::info!("Volume Down Media Key Released");
                    hotkey_state_clone.volume_down_active.store(false, Ordering::SeqCst);
                }
            } else if event.id() == mute.id()
                || event.id() == mute2.id()
                || event.id() == mute4.id()
                || event.id() == mute3.id()
                || event.id() == mute5.id()
            {
                if event.state() == HotKeyState::Pressed {
                    log::info!("Mute Media Key Pressed");
                    handle_session_toggle_mute("master", window.clone());
                }
            }
        }
    });
}

fn register_mixer_hotkey(app: &AppHandle) {
    let config = get_config();
    let hotkey = config.mixer.hotkey.clone();
    if let Some(hotkey) = hotkey {
        let app_handle = app.clone();
        app.global_shortcut_manager()
            .register(&hotkey, move || {
                toggle_window(&app_handle);
            })
            .unwrap_or_else(|e| log::error!("Failed to register hotkey: {}", e));
    }
}

fn override_media_keys(window: WindowWrapper, app: AppHandle) {
    log::info!("Initializing key listeners");

    let config = get_config();

    // Parse all key chords from config once
    // let parsed_keybinds: Vec<(HashSet<KeybdKey>, String, String)> = config
    //     .sessions
    //     .iter()
    //     .flat_map(|session| {
    //         session.keybinds.iter().flat_map(|keybinds| {
    //             keybinds.iter().filter_map(|keybind| {
    //                 let chord = keyboard::parse_key_chord(&keybind.key);
    //                 if !chord.is_empty() {
    //                     Some((chord, keybind.action.clone(), session.name.clone()))
    //                 } else {
    //                     None
    //                 }
    //             })
    //         })
    //     })
    //     .collect();

    let hotkey = config.mixer.hotkey.clone();
    let chord = keyboard::parse_key_chord(&hotkey.unwrap());

    // Custom key bindings from config
    KeybdKey::bind_all({
        // let window = window.clone();
        // let parsed_keybinds = parsed_keybinds.clone();
        move |event| {
            if !chord.is_empty() {
                if chord.contains(&event) && keyboard::is_chord_pressed(&chord) {
                    log::info!("Matched key chord: {:?}", chord);
                    toggle_window(&app);
                }
            }

            // if !keyboard::is_modifier_key(event) {
            //     for (chord, action, session_name) in &parsed_keybinds {
            //         if chord.contains(&event) && keyboard::is_chord_pressed(chord) {
            //             log::info!("Matched key chord: {:?}", chord);
            //             handle_action(action, session_name, window.clone());
            //         }
            //     }
            // }
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

fn toggle_window(app: &AppHandle) {
    if let Some(window) = app.get_window("mixer_window") {
        log::info!("Toggling window");
        let is_visible = window.is_visible().unwrap();
        api::events::emit_mixer_visibility_change_event(!is_visible, &window);
    } else {
        log::info!("Creating new window");
        window_manager::create_new_window(app);
    }
}

fn handle_session_up(session_name: &str, window: WindowWrapper) {
    log::info!("Session up: {}", session_name);
    let current_vol = volume_manager::get_session_volume(session_name);
    volume_manager::set_session_volume(session_name, current_vol + 2);

    let sessions = volume_manager::get_sessions(session_name);
    for session in sessions {
        window.show_and_emit(AppEvent::VolumeChange, session);
    }
}

fn handle_session_down(session_name: &str, window: WindowWrapper) {
    log::info!("Session down: {}", session_name);
    let current_vol = volume_manager::get_session_volume(session_name);
    volume_manager::set_session_volume(session_name, current_vol - 2);

    let sessions = volume_manager::get_sessions(session_name);
    for session in sessions {
        window.show_and_emit(AppEvent::VolumeChange, session);
    }
}

fn handle_session_toggle_mute(session_name: &str, window: WindowWrapper) {
    log::info!("Session toggle mute: {}", session_name);
    volume_manager::toggle_session_mute(session_name);

    let sessions = volume_manager::get_sessions(session_name);
    for session in sessions {
        window.show_and_emit(AppEvent::VolumeChange, session);
    }
}

// fn handle_action(action: &str, session_name: &str, window: WindowWrapper) {
//     match action {
//         "VolumeUp" => handle_session_up(session_name, window),
//         "VolumeDown" => handle_session_down(session_name, window),
//         "ToggleMute" => handle_session_toggle_mute(session_name, window),
//         _ => log::warn!("Unknown action: {}", action),
//     }
// }

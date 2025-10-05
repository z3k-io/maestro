use tauri::{AppHandle, Emitter, Manager};

use crate::{config::Config, models::audio_session::AudioSession, services::window_service};

#[derive(Clone, Copy)]
pub enum AppEvent {
    VolumeChange,
    MixerVisibilityChange,
    ConfigChange,
    ThemeChange,
    WindowHidden,
}

impl AppEvent {
    pub fn as_str(&self) -> &'static str {
        match self {
            AppEvent::VolumeChange => "volume-change-event",
            AppEvent::MixerVisibilityChange => "mixer-visibility-change-event",
            AppEvent::ConfigChange => "config-change-event",
            AppEvent::ThemeChange => "theme-change-event",
            AppEvent::WindowHidden => "window-hidden-event",
        }
    }
}

pub fn emit_volume_change_event(audio_session: &AudioSession, app_handle: AppHandle) {
    app_handle.emit(AppEvent::VolumeChange.as_str(), audio_session).unwrap();

    // Todo: this is outside the scope of what this class should be handling
    let mixer_visible = app_handle.get_webview_window("mixer").unwrap().is_visible().unwrap();
    if mixer_visible {
        return;
    }

    window_service::show_overlay(app_handle.clone());
}

pub fn emit_mixer_visibility_change_event(visible: bool, app_handle: AppHandle) {
    app_handle.emit(AppEvent::MixerVisibilityChange.as_str(), visible).unwrap();
}

pub fn emit_config_change_event(config: &Config, app_handle: AppHandle) {
    app_handle.emit(AppEvent::ConfigChange.as_str(), config).unwrap();
    app_handle
        .emit(AppEvent::ThemeChange.as_str(), config.system.theme.clone())
        .unwrap();
}

pub fn emit_window_hidden_event(app_handle: AppHandle) {
    app_handle.emit(AppEvent::WindowHidden.as_str(), ()).unwrap();
}

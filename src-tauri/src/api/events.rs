use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager};

use crate::{models::audio_session::AudioSession, services::window_service};

#[derive(Clone, Copy)]
pub enum AppEvent {
    VolumeChange,
    MixerVisibilityChange,
}

#[derive(Serialize, Deserialize)]
pub struct VolumeChangePayload {
    pub session_name: String,
    pub volume: f32,
}

#[derive(Serialize, Deserialize)]
pub struct MixerVisibilityPayload {
    pub visible: bool,
}

impl AppEvent {
    pub fn as_str(&self) -> &'static str {
        match self {
            AppEvent::VolumeChange => "volume-change-event",
            AppEvent::MixerVisibilityChange => "mixer-visibility-change-event",
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

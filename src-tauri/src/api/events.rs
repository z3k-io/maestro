use serde::{Deserialize, Serialize};
use tauri::Window;

use crate::models::audio_session::AudioSession;

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

pub fn emit_volume_change_event(audio_session: &AudioSession, window: &Window) {
    window.emit(AppEvent::VolumeChange.as_str(), audio_session).unwrap();
}

pub fn emit_mixer_visibility_change_event(visible: bool, window: &Window) {
    window.emit(AppEvent::MixerVisibilityChange.as_str(), visible).unwrap();
}

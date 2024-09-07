#[derive(Clone, Copy)]
pub enum AppEvent {
    VolumeChange,
    MixerVisibilityChange,
}

impl AppEvent {
    pub fn as_str(&self) -> &'static str {
        match self {
            AppEvent::VolumeChange => "volume-change-event",
            AppEvent::MixerVisibilityChange => "mixer-visibility-change-event",
        }
    }
}

use windows_volume_control::session::Session;

use crate::utils::icon_service;

#[derive(serde::Serialize)]
pub struct AudioSession {
    pub name: String,
    pub volume: i32,
    pub mute: bool,
    pub icon: Option<String>,
}

impl AudioSession {
    pub fn from_session(session: &Box<dyn Session>) -> Self {
        unsafe {
            AudioSession {
                name: session.get_name().to_string(),
                volume: (session.get_volume() * 100.0).round() as i32,
                mute: session.get_mute(),
                icon: icon_service::get_icon(session.get_pid()),
            }
        }
    }
}

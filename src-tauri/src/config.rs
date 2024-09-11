use serde::Deserialize;
use std::sync::Arc;
use std::{fs::File, io::Read, sync::OnceLock};

#[derive(Debug, Deserialize, Clone)]
pub struct ArduinoConfig {
    pub com_port: String,
    pub baud_rate: u32,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct KeybindConfig {
    pub key: String,
    pub action: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MixerConfig {
    pub hotkey: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct SessionConfig {
    pub name: String,
    pub encoder: u8,
    pub keybinds: Option<Vec<KeybindConfig>>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub arduino: ArduinoConfig,
    pub sessions: Vec<SessionConfig>,
    pub mixer: MixerConfig,
}

static CONFIG: OnceLock<Arc<Config>> = OnceLock::new();

fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let file_path = std::env::current_dir()?.join("config.yaml");

    log::info!("Loading config from: {}", file_path.to_str().unwrap());

    let mut file = File::open(file_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    log::info!("Config loaded: {}", contents);

    let config: Config = serde_yaml::from_str(&contents)?;

    Ok(config)
}

// Return all defined sessions except for 'other'
pub fn get_defined_session_names() -> Vec<String> {
    let config = get_config();
    let session_names: Vec<String> = config
        .sessions
        .iter()
        .filter(|session| session.name.to_lowercase() != "other")
        .map(|session| session.name.clone())
        .collect();
    return session_names.iter().map(|name| name.to_lowercase()).collect();
}

pub fn get_config() -> Arc<Config> {
    CONFIG
        .get_or_init(|| Arc::new(load_config().expect("Failed to load config")))
        .clone()
}

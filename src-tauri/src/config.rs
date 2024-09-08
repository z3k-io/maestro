use serde::Deserialize;
use std::sync::Arc;
use std::{fs::File, io::Read, sync::OnceLock};

// TODO: Make this work automagically, shouldn't need to manually change for debugger.

const CONFIG_PATH: &'static str = "config.yaml";
// const CONFIG_PATH: &'static str = "src-tauri/config.yaml";

#[derive(Debug, Deserialize, Clone)]
pub struct ArduinoConfig {
    pub com_port: String,
    pub baud_rate: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct KeybindConfig {
    pub key: String,
    pub action: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MixerConfig {
    pub hotkey: Option<String>,
}

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
    log::info!("Current working directory: {:?}", std::env::current_dir()?);
    log::info!("Loading config from: {}", CONFIG_PATH);

    let mut file = File::open(CONFIG_PATH)?;

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
        .get_or_init(|| {
            let config = load_config().expect("Failed to load config");
            log::info!("Loaded config: {:?}", config);
            Arc::new(config)
        })
        .clone()
}

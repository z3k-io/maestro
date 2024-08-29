use serde::Deserialize;
use std::sync::Arc;
use std::{fs::File, io::Read, sync::OnceLock};

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub com_port: String,
    pub baud_rate: u32,
    pub inputs: Vec<String>,
}

// Static variable to hold the config singleton
static CONFIG: OnceLock<Arc<Config>> = OnceLock::new();

// const CONFIG_PATH: &'static str = "src-tauri/config.yaml";
const CONFIG_PATH: &'static str = "config.yaml";

// Function to initialize the config
fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    // print working dir
    log::info!("Current working directory: {:?}", std::env::current_dir()?);
    log::info!("Loading config from: {}", CONFIG_PATH);
    let mut file = File::open(CONFIG_PATH)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    log::info!("Config loaded: {}", contents);

    let config: Config = serde_yaml::from_str(&contents)?;

    Ok(config)
}

// Public function to get the config
pub fn get_config() -> Arc<Config> {
    CONFIG
        .get_or_init(|| {
            let config = load_config().expect("Failed to load config");
            log::info!("Loaded config: {:?}", config);
            Arc::new(config)
        })
        .clone()
}

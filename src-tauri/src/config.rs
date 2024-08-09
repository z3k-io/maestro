use serde::Deserialize;
use std::sync::Arc;
use std::sync::Mutex;
use std::{fs::File, io::Read, sync::OnceLock};

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub com_port: String,
    pub baud_rate: u32,
    pub inputs: Vec<String>,
}

// Static variable to hold the config singleton
static CONFIG: OnceLock<Arc<Config>> = OnceLock::new();

// Function to initialize the config
fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config: Config = serde_yaml::from_str(&contents)?;

    Ok(config)
}

// Public function to get the config
pub fn get_config() -> Arc<Config> {
    CONFIG
        .get_or_init(|| {
            let config = load_config("../config.yaml").expect("Failed to load config");
            println!("Loaded config: {:?}", config);
            Arc::new(config)
        })
        .clone()
}

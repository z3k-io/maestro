use serde::Deserialize;
use std::env;
use std::path::PathBuf;
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

static IS_DEUBG: bool = false;

fn get_config_path() -> &'static str {
    let current_dir: PathBuf = env::current_dir().expect("Failed to get current directory");
    println!("Current working directory: {}", current_dir.display());

    if IS_DEUBG {
        return "../config.yaml";
    } else {
        return "../config.yaml";
    }
}

// Function to initialize the config
fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = get_config_path();
    println!("Loading config from: {}", config_path);
    let mut file = File::open(config_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config: Config = serde_yaml::from_str(&contents)?;

    Ok(config)
}

// Public function to get the config
pub fn get_config() -> Arc<Config> {
    CONFIG
        .get_or_init(|| {
            let config = load_config().expect("Failed to load config");
            println!("Loaded config: {:?}", config);
            Arc::new(config)
        })
        .clone()
}

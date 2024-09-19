use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter};
use yaml_rust::{Yaml, YamlEmitter, YamlLoader};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArduinoConfig {
    pub enabled: bool,
    pub com_port: String,
    pub baud_rate: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeybindConfig {
    pub key: String,
    pub action: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MixerConfig {
    pub enabled: bool,
    pub hotkey: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemConfig {
    pub autostart: bool,
    pub show_console: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionConfig {
    pub name: String,
    pub keybinds: Option<Vec<KeybindConfig>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub arduino: ArduinoConfig,
    pub sessions: Vec<SessionConfig>,
    pub mixer: MixerConfig,
    pub system: SystemConfig,
}

static CONFIG: Lazy<Arc<Mutex<Config>>> = Lazy::new(|| Arc::new(Mutex::new(load_config())));

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

pub fn get_config() -> Config {
    CONFIG.lock().unwrap().clone()
}

pub fn set_config(config: Config, app_handle: &AppHandle) {
    let pretty_config = to_string_pretty(&config).unwrap();
    log::info!("Saving config:\n{}", pretty_config);

    save_config(&config).unwrap();

    let reloaded_config = load_config();

    let mut config_guard = CONFIG.lock().unwrap();
    *config_guard = reloaded_config;

    app_handle.emit("config_changed", config).unwrap();

    log::info!("Config reload complete.");
}

fn save_config(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = config_file_path()?;
    let mut file_content = String::new();
    if file_path.exists() {
        fs::File::open(&file_path)?.read_to_string(&mut file_content)?;
    }

    let mut yaml_docs = YamlLoader::load_from_str(&file_content).unwrap_or_else(|_| vec![Yaml::Hash(yaml_rust::yaml::Hash::new())]);
    let yaml_config = &mut yaml_docs[0];

    // Update the YAML with new values from the config
    update_yaml_from_config(yaml_config, config);

    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(yaml_config)?;
    }

    fs::File::create(&file_path)?.write_all(out_str.as_bytes())?;
    Ok(())
}

fn load_config() -> Config {
    let file_path = config_file_path().unwrap();
    let mut file_content = String::new();
    fs::File::open(&file_path).unwrap().read_to_string(&mut file_content).unwrap();

    let yaml_docs = YamlLoader::load_from_str(&file_content).unwrap();
    let yaml_config = &yaml_docs[0];

    let config: Config = serde_yaml::from_value(yaml_to_value(yaml_config)).unwrap();
    return config;
}

fn config_file_path() -> Result<PathBuf, std::io::Error> {
    Ok(std::env::current_dir()?.join("config.yaml"))
}

fn yaml_to_value(yaml: &Yaml) -> serde_yaml::Value {
    match yaml {
        Yaml::Real(s) => serde_yaml::Value::Number(s.parse().unwrap()),
        Yaml::Integer(i) => serde_yaml::Value::Number((*i).into()),
        Yaml::String(s) => serde_yaml::Value::String(s.clone()),
        Yaml::Boolean(b) => serde_yaml::Value::Bool(*b),
        Yaml::Array(a) => serde_yaml::Value::Sequence(a.iter().map(yaml_to_value).collect()),
        Yaml::Hash(h) => serde_yaml::Value::Mapping(h.iter().map(|(k, v)| (yaml_to_value(k), yaml_to_value(v))).collect()),
        Yaml::Null => serde_yaml::Value::Null,
        _ => unreachable!(),
    }
}

fn update_yaml_from_config(yaml: &mut Yaml, config: &Config) {
    if let Yaml::Hash(ref mut hash) = *yaml {
        update_yaml_field(hash, "arduino", &config.arduino);
        update_yaml_field(hash, "sessions", &config.sessions);
        update_yaml_field(hash, "mixer", &config.mixer);
        update_yaml_field(hash, "system", &config.system);
    }
}

fn update_yaml_field<T: Serialize>(hash: &mut yaml_rust::yaml::Hash, key: &str, value: &T) {
    let yaml_value = serde_yaml::to_value(value).unwrap();
    let yaml_key = Yaml::String(key.to_string());
    hash.insert(yaml_key, value_to_yaml(&yaml_value));
}

fn value_to_yaml(value: &serde_yaml::Value) -> Yaml {
    match value {
        serde_yaml::Value::Null => Yaml::Null,
        serde_yaml::Value::Bool(b) => Yaml::Boolean(*b),
        serde_yaml::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Yaml::Integer(i)
            } else if let Some(f) = n.as_f64() {
                Yaml::Real(f.to_string())
            } else {
                Yaml::Null
            }
        }
        serde_yaml::Value::String(s) => Yaml::String(s.clone()),
        serde_yaml::Value::Sequence(seq) => Yaml::Array(seq.iter().map(value_to_yaml).collect()),
        serde_yaml::Value::Mapping(m) => {
            let mut hash = yaml_rust::yaml::Hash::new();
            for (k, v) in m {
                hash.insert(value_to_yaml(k), value_to_yaml(v));
            }
            Yaml::Hash(hash)
        }
        serde_yaml::Value::Tagged(tagged) => value_to_yaml(&tagged.value),
    }
}

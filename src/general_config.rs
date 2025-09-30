use notify::Error;
use serde::{Deserialize, Serialize};

use crate::general_config;

const GENERAL_CONFIG_FILE: &str = "~/comiti/comiti.json";

#[derive(Serialize, Deserialize)]
pub struct GeneralConfig {
    pub directories: Vec<String>,
}

fn expand_tilde(path: &str) -> std::path::PathBuf {
    if let Some(stripped) = path.strip_prefix("~") {
        if let Some(home) = dirs::home_dir() {
            return home.join(stripped.trim_start_matches('/'));
        }
    }
    std::path::PathBuf::from(path)
}

pub fn load() -> Result<GeneralConfig, serde_json::Error> {
    let path = expand_tilde(GENERAL_CONFIG_FILE);
    let data = match std::fs::read_to_string(&path) {
        Ok(content) => content,
        Err(_) => create(),
    };

    let config: GeneralConfig = match serde_json::from_str(&data) {
        Ok(cfg) => cfg,
        Err(e) => {
            if e.is_eof() {
                serde_json::from_str(&create()).unwrap()
            } else {
                return Err(e);
            }
        }
    };

    write(serde_json::to_string(&config).unwrap()).unwrap();

    Ok(config)
}

fn create() -> String {
    let default_config = GeneralConfig {
        directories: vec![],
    };
    serde_json::to_string(&default_config).unwrap()
}

fn write(content: String) -> std::io::Result<()> {
    let path = expand_tilde(GENERAL_CONFIG_FILE);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut file = std::fs::File::create(path)?;
    std::io::Write::write_all(&mut file, content.as_bytes())?;

    Ok(())
}

pub fn add_dir(path: String) -> Result<(), Error> {
    let mut general_config = load().unwrap();

    general_config.directories.push(path);

    println!("{:?}", general_config.directories);

    write(serde_json::to_string(&general_config).unwrap()).unwrap();

    Ok(())
}

use serde::{Deserialize, Serialize};

const GENERAL_CONFIG_FILE: &str = "commitr.json";

#[derive(Serialize, Deserialize)]
pub struct GeneralConfig {
    pub directories: Vec<String>,
}

pub fn load() -> Result<GeneralConfig, serde_json::Error> {
    let data = match std::fs::read_to_string(GENERAL_CONFIG_FILE) {
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
    let mut file = std::fs::File::create(GENERAL_CONFIG_FILE)?;
    std::io::Write::write_all(&mut file, content.as_bytes())?;

    Ok(())
}

use serde::{Deserialize, Serialize};

const USER_CONFIG_FILE: &str = "commitr_config.json";

#[derive(Serialize, Deserialize)]
pub struct UserConfig {
    file_formats: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UserConfigWithPath {
    path: String,
    file_formats: Vec<String>,
}

pub fn load(path: String) -> Result<UserConfig, serde_json::Error> {
    let full_path = format!("{}/{}", path, USER_CONFIG_FILE);
    let data = match std::fs::read_to_string(full_path) {
        Ok(content) => content,
        Err(_) => create(),
    };

    let config: UserConfig = match serde_json::from_str(&data) {
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

pub fn load_all(paths: Vec<String>) -> Result<Vec<UserConfigWithPath>, serde_json::Error> {
    let mut all_configs_with_paths: Vec<UserConfigWithPath> = Vec::new();

    for path in paths {
        let full_path = format!("{}/{}", path, USER_CONFIG_FILE);
        let data = match std::fs::read_to_string(&full_path) {
            Ok(content) => content,
            Err(_) => create(),
        };

        let config: UserConfig = match serde_json::from_str(&data) {
            Ok(cfg) => cfg,
            Err(e) => {
                if e.is_eof() {
                    serde_json::from_str(&create()).unwrap()
                } else {
                    return Err(e);
                }
            }
        };

        all_configs_with_paths.push(UserConfigWithPath {
            path: path.clone(),
            file_formats: config.file_formats.clone(),
        });
    }

    Ok(all_configs_with_paths)
}

pub fn create() -> String {
    let default_config = UserConfig {
        file_formats: vec![],
    };
    serde_json::to_string(&default_config).unwrap()
}

fn write(content: String) -> std::io::Result<()> {
    let mut file = std::fs::File::create(USER_CONFIG_FILE)?;
    std::io::Write::write_all(&mut file, content.as_bytes())?;

    Ok(())
}

use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{
    fmt::write,
    fs::{File, read_to_string},
    io::Write,
};

#[derive(Serialize, Deserialize)]
struct GeneralConfig {
    directories: Vec<DirectoryConfig>,
}

#[derive(Serialize, Deserialize)]
struct DirectoryConfig {
    path: String,
    file_formats: Vec<String>,
}

pub fn load_general_config() {
    let data = match read_to_string("./commitr_config.json") {
        Ok(content) => content,
        Err(_) => create_general_config(),
    };

    let config: GeneralConfig = serde_json::from_str(&data).unwrap();
    write_general_config_file(serde_json::to_string(&config).unwrap()).unwrap();

    println!("{}", serde_json::to_string(&config).unwrap())
}

pub fn create_general_config() -> String {
    let default_config = GeneralConfig {
        directories: vec![],
    };
    serde_json::to_string(&default_config).unwrap()
}

fn write_general_config_file(content: String) -> std::io::Result<()> {
    let mut file = File::create("commitr_config.json")?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

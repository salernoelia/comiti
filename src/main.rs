mod general_config;
mod user_config;

fn main() {
    let general_config = general_config::load().unwrap();

    println!("{}", serde_json::to_string(&general_config).unwrap());

    let user_config = user_config::load(String::from(".")).unwrap();
    let all_user_configs = user_config::load_all(general_config.directories).unwrap();

    println!("{}", serde_json::to_string(&user_config).unwrap());
    println!("{}", serde_json::to_string(&all_user_configs).unwrap());
}

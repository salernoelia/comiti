mod cli;
mod general_config;
mod git;
mod user_config;

fn main() {
    match git::Git::fetch(".") {
        Ok(_) => println!("Fetch succeeded"),
        Err(e) => eprintln!("Fetch failed: {}", e),
    }

    match git::Git::add_all(".") {
        Ok(_) => println!("Add succeeded"),
        Err(e) => eprintln!("Add failed: {}", e),
    }

    match git::Git::commit(".", "commited from comittr") {
        Ok(_) => println!("Commit succeeded"),
        Err(e) => eprintln!("Commit failed: {}", e),
    }

    match git::Git::push(".") {
        Ok(_) => println!("Push succeeded"),
        Err(e) => eprintln!("Push failed: {}", e),
    }

    cli::init();
    let general_config = general_config::load().unwrap();

    println!("{}", serde_json::to_string(&general_config).unwrap());

    let user_config = user_config::load(String::from(".")).unwrap();
    let all_user_configs = user_config::load_all(general_config.directories).unwrap();

    println!("{}", serde_json::to_string(&user_config).unwrap());
    println!("{}", serde_json::to_string(&all_user_configs).unwrap());
}

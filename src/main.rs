mod setting;

fn main() {
    match setting::Configs::init() {
        Ok(config) => {
            println!("Config: {:?}", config);
            println!("Running on port: {}", config.port);
            println!("Environment: {}", config.env);
        }
        Err(e) => {
            eprintln!("Failed to load config: {}", e);
            std::process::exit(1);
        }
    }
}
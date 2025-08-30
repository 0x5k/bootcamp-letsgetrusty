use log::debug;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct PwnConfig {
    name: String,
}

fn parse_pwnagotchi_config(file: &str) -> Result<PwnConfig, Box<dyn std::error::Error>> {
    debug!("Attempting to parse config from: {}", file);
    let toml_str = fs::read_to_string(file)?;
    let config: PwnConfig = toml::from_str(&toml_str)?;
    debug!("Successfully parsed config: {:?}", config);
    Ok(config)
}

fn main() {
    env_logger::init();
    match parse_pwnagotchi_config("/etc/pwnagotchi/config.toml") {
        Ok(config) => {
            println!("Parsed Pwnagotchi config:");
            println!("Name: {}", config.name);
        }
        Err(e) => eprintln!("Error parsing config: {}", e),
    }
}

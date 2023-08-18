// filename: config.rs

use std::error::Error;
use std::fs;

use rust_register_center::config::Config;

pub static CONFIG_FILE: &str = "tests/config.xml";
pub static CONFIG_FILE_UNEXPECTED_END_OF_STREAM: &str = "tests/config_unexpected_end_of_stream.xml";
pub static PROJECT_ROOT: &str = env!("CARGO_MANIFEST_DIR");

fn load_config(filename: &str) -> Result<Config, Box<dyn Error>> {
    let xml = std::fs::read_to_string(filename)?;
    let config: Config = serde_xml_rs::from_str(&xml)?;
    Ok(config)
}

#[allow(unused)]
fn file_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

#[allow(unused)]
fn is_valid(path: &str) -> bool {
    file_exists(path)
        && path.ends_with(".xml")
}

#[actix_web::main]
async fn main() {
    let config_file = vec![PROJECT_ROOT, CONFIG_FILE_UNEXPECTED_END_OF_STREAM]
        .join("/")
        .to_string();
    let err = load_config(&config_file).unwrap_err();
    println!("{}", err);
}

#[cfg(test)]
mod tests {
    // Tests for config module

    use super::*;

    #[test]
    fn test_load_config_ok() {
        let config_file = vec![PROJECT_ROOT, CONFIG_FILE].join("/").to_string();
        println!("config: {}", config_file);
        let config = load_config(&config_file).unwrap();
        assert_eq!(config.host, "localhost");
    }

    #[test]
    fn test_load_config_file_not_found() {
        let config_file = vec![PROJECT_ROOT, CONFIG_FILE, "not_exists.xml"]
            .join("/")
            .to_string();
        let result = file_exists(&config_file);
        assert!(!result);
    }

    #[test]
    fn test_load_config_invalid_xml() {
        let config_file = vec![PROJECT_ROOT, CONFIG_FILE_UNEXPECTED_END_OF_STREAM]
            .join("/")
            .to_string();
        let err = load_config(&config_file).unwrap_err();
        assert!(err.to_string().contains("Unexpected end of stream"));
    }
}

// filename: config.rs

use std::error::Error;
use std::fs;

use rust_register_center::config::Config;

pub static CONFIG_FILE: &str = "tests/resources/config.xml";
pub static CONFIG_FILE_UNEXPECTED_END_OF_STREAM: &str = "tests/resources/config_unexpected_end_of_stream.xml";
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

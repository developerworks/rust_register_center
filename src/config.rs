// filename: config.rs

mod struct_defs;

use std::error::Error;
use std::fs;

pub use struct_defs::Config;

// #[allow(unused)]
pub static CONFIG_FILE: &str = "tests/config.xml";

// #[allow(unused)]
pub static PROJECT_ROOT: &str = env!("CARGO_MANIFEST_DIR");

#[allow(unused)]
pub fn load_config(filename: &str) -> Result<Config, Box<dyn Error>> {
    let xml = std::fs::read_to_string(filename)?;
    let config: Config = serde_xml_rs::from_str(&xml)?;
    Ok(config)
}
#[allow(unused)]
pub fn file_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

// fn is_valid(path: &str) -> bool {
//     file_exists(path)
//         && path.ends_with(".xml")
// }


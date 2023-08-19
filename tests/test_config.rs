use rust_register_center::config::{PROJECT_ROOT, CONFIG_FILE, load_config, file_exists};

// Tests for config module
// #[allow(unused)]
pub static CONFIG_FILE_UNEXPECTED_END_OF_STREAM: &str = "tests/config_unexpected_end_of_stream.xml";


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


use serde_json::json;

use rust_register_center::service_configuration::ServiceConfiguration;


#[test]
fn test_convert_to_yaml() {
    let json = json!({"key1": "value1", "key2": "value2"});
    let yaml = ServiceConfiguration::convert_to_yaml(json.to_string());
    assert_eq!(
        yaml,
        r#"key1: value1
key2: value2
"#
    );
}

#[test]
fn test_convert_to_json() {
    let yaml = r#"key1: value1
key2: value2"#;
    let json = ServiceConfiguration::convert_to_json(yaml.to_string());
    assert_eq!(json, r#"{"key1":"value1","key2":"value2"}"#);
}

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[allow(unused)]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
struct ServiceConfiguration {}
impl ServiceConfiguration {
    /// Initialize the ServiceConfiguration
    #[allow(unused)]
    pub fn new() -> Self {
        Self {}
    }

    /// Convert json string to yaml string
    #[allow(unused)]
    pub fn convert_to_yaml(json_str: String) -> String {
        let json: Value = serde_json::from_str(&json_str).unwrap();
        serde_yaml::to_string(&json).unwrap()
    }

    /// Convert yaml string to json string
    #[allow(unused)]
    pub fn convert_to_json(yaml_str: String) -> String {
        let json_data: serde_json::Value = serde_yaml::from_str(&yaml_str).unwrap();
        serde_json::to_string(&json_data).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

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
}

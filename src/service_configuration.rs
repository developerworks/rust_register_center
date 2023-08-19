use serde::{Deserialize, Serialize};
use serde_json::Value;

#[allow(unused)]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ServiceConfiguration {}
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


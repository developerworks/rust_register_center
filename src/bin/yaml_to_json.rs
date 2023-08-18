use serde_yaml::from_str;
use serde_json::to_string_pretty;

fn main() {
    let yaml_data = r#"
    name: John Doe
    age: 43
    address:
      street: 10 Main St
      city: Anytown
      state: CA
      zip: 12345
    "#;
    
    let json_data: serde_json::Value = from_str(yaml_data).unwrap();
    
    let json_string = to_string_pretty(&json_data).unwrap();
    println!("{}", json_string)
}
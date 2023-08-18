use serde_json::json;

fn main() {
    let json_data = json!({
        "name": "John Doe",
        "age": 43,
        "address": {
            "street": "10 Main St",
            "city": "Anytown",
            "state": "CA",
            "zip": 12345
        }
    });
    
    let yaml_data = serde_yaml::to_string(&json_data).unwrap();
    println!("{}", yaml_data)
}
use std::fs;
use crate::JsonValue;

pub trait Resource {
    fn load(file_name: &str) -> Self;
    fn load_json(file_name: &str) -> JsonValue {
        let contents = fs::read_to_string(file_name)
                    .expect(&format!("le fichier '{}' est illisible", file_name));
    json::parse(&contents).expect("JSON Mal formé")
    }
    fn get_json(&self) -> JsonValue;
    fn get(&self, field: &str) -> JsonValue{
        self.get_json()[field].clone()
    }
}

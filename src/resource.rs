use std::fs;
use crate::JsonValue;
use json::JsonError;

pub trait Resource {
    fn load(file_name: &str) -> Self;
    fn load_json(file_name: &str) -> Result<JsonValue, JsonError> {
        let contents = fs::read_to_string(file_name)
                    .expect(&format!("le fichier '{}' est illisible", file_name));
        json::parse(&contents)
    }
    fn get_json(&self) -> JsonValue;
    fn get(&self, field: &str) -> JsonValue{
        self.get_json()[field].clone()
    }
}

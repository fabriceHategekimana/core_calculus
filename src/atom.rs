use json::JsonValue;
use crate::resource::Resource;

#[derive(Clone)]
pub struct Atom(pub JsonValue);

impl Atom {
    pub fn new(value: JsonValue) -> Atom {
        Atom(value)
    }

    pub fn check(self) -> Self {
        // TODO: see what need to be checked
        self
    }

    pub fn name(&self) -> String {
        self.0["name"].to_string()
    }

    pub fn formula(&self) -> String {
        self.0["formula"].to_string()
    }

    pub fn evaluation(&self) -> Vec<(String, Vec<String>)> {
        self.0["evaluation"].members()
            .map(|x| (x["name"].to_string(), Self::get_rule(x)))
            .collect()
    }

    fn get_rule(j: &JsonValue) -> Vec<String> {
        j["rule"].members().map(|x| x.to_string()).collect()
    }

    pub fn typing(&self) -> Vec<(String, Vec<String>)> {
        self.0["typing"].members()
            .map(|x| (x["name"].to_string(), Self::get_rule(x)))
            .collect()
    }
}

impl Resource for Atom {
    fn load(file_name: &str) -> Atom {
        Atom(Self::load_json(file_name))
    }

    fn get_json(&self) -> JsonValue {
        self.0.clone()
    }
}

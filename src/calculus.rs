use json::JsonValue;
use crate::resource::Resource;

#[derive(Clone)]
pub struct Calculus(pub JsonValue);

impl Calculus {
    pub fn check(self) -> Self {
        // TODO: see what need to be checked
        self
    }
}

impl Resource for Calculus {
   fn load(file_name: &str) -> Calculus {
       Calculus(Self::load_json(file_name).expect(&format!("There is an error in '{}'", file_name))).check()
   } 

   fn get_json(&self) -> JsonValue {
       self.0.clone()
   }
}

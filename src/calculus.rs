use json::JsonValue;

#[derive(Clone)]
pub struct Calculus(pub JsonValue);

impl Calculus {
    pub fn check(self) -> Self {
        // TODO: see what need to be checked
        self
    }

    pub fn get_members(&self) -> Vec<String> {
        self.0["groups"].members()
            .flat_map(|x| x["members"].members()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>())
            .collect()
    }
}

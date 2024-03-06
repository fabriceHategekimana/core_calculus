use crate::Atom;
use crate::Calculus;
use crate::treat_variables;
use json::JsonValue;
use crate::json_to_array;
use crate::load;

pub trait Renderer {
    fn title(&self) -> String;
    fn calculus(&self) -> Calculus;
    fn atoms(&self) -> Vec<Atom>;
    fn syntax(&self) -> String;
    fn evaluation(&self) -> String;
    fn typing(&self) -> String;
    fn render(self) -> String where Self: Sized {
        [self.title(),
         self.syntax(),
         self.evaluation(),
         self.typing()].iter()
         .map(String::from)
         .collect()
    }
}

pub struct Typst(pub Calculus, pub Vec<Atom>);

impl Renderer for Typst {
    fn calculus(self: &Typst) -> Calculus {
        self.0.clone()
    }

    fn atoms(self: &Typst) -> Vec<Atom> {
        self.1.clone()
    }

   fn title(self: &Typst) -> String {
       format!( "= {}\n\n",
           self.calculus().0["name"].to_string())
   } 

   fn syntax(self: &Typst) -> String {
       let body = self.calculus().0["groups"].members()
       .map(|x| render_group_member(x, &self.atoms()))
       .collect::<String>();
       format!("== Syntax\n{}\n", body)
   }

   fn evaluation(self: &Typst) -> String {
       let body: String = self.atoms().iter()
           .flat_map(|x| x.evaluation())
           .map(|(name, rules)| format!("\t$ {} & \"{}\" $\n", treat_variables(&to_div(&rules)), name))
           .collect();
       format!("== Evaluation\n{}\n", body)
   }

   fn typing(self: &Typst) -> String {
       let body: String = self.atoms().iter()
           .flat_map(|x| x.typing())
           .map(|(name, rules)| format!("\t$ {} & \"{}\" $\n", treat_variables(&to_div(&rules)), name))
           .collect();
       format!("== Typing\n{}\n", body)
   }

}

fn type_rule(rule: JsonValue) -> String {
    "type rule\n".to_string()
}

fn render_group_member(member: &JsonValue, atoms: &[Atom]) -> String{
    let intro = format!("{} ::= {}\n", member["symbol"], member["name"]);
    let rest = json_to_array(&member["members"])
        .iter()
        .map(|x| (get_formula(&x.to_string(), atoms), x))
        .map(|x| format!("\t${}$\t\t{}\n\n", x.0, x.1))
        .collect::<String>();
    format!("{}\n{}\n", intro, rest)
}

fn get_formula(s: &str, atoms: &[Atom]) -> String {
    atoms.iter().filter(|x| x.name() == s)
        .map(|x| x.formula())
        .collect()
}

fn to_div(elements: &[String]) -> String {
    let top = elements.iter().skip(1)
        .map(|x| format!("{}; ", x))
        .collect::<String>();
    format!("({})/({}) ", top, elements.get(0).unwrap())
}

fn render_member(m: &JsonValue) -> String {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test(){
        let res = get_formula(&"type abstraction".to_string(), 
                              &["src/atoms/atom1.json"].iter().map(|&x| Atom(load(&x.to_string()))).collect());
        assert_eq!("lambda x . y", res);
    }
}

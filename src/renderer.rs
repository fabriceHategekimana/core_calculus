use crate::Atom;
use crate::Calculus;
use crate::resource::Resource;
use json::JsonValue;
use crate::file_manager::load;
use std::{fs::File,io::prelude::*};
use std::process::Command;
use crate::file_manager::add_extension;
use regex::Regex;

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
           self.calculus().get("name").to_string())
   } 

   fn syntax(self: &Typst) -> String {
       let body = self.calculus().get("groups").members()
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
    let rest = member["members"]
        .members()
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

fn render_pdf(name: &str) -> () {
    Command::new("quarto")
        .args(["typst", "compile", &format!("{}.typ", name)])
        .output()
        .expect("zut! ");
}

pub fn render_typ(calculus: Calculus, atoms: &[Atom], file_name: &str) -> () {
    let content = Typst(calculus, atoms.to_vec()).render();
    write_content(&add_extension(file_name, "typ"), &content);
    render_pdf(&file_name);
}

fn write_content(file_name: &str, content: &str) -> () {
    File::create(file_name)
        .expect(&format!("fichier '{}' introuvable", file_name))
        .write_all(content.as_bytes())
        .expect(&format!("impossible d'écrire dans le fichier '{}'", file_name));
}

fn treat_variables(input: &str) -> String {
    let re = Regex::new(r"([a-zA-Z]+[0-9]+)").expect("Erreur lors de la création de l'expression régulière");
    let resultat = re.replace_all(input, "\"$1\"");
    resultat.to_string()
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


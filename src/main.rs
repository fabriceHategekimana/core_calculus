#![allow(dead_code, unused_variables, unused_imports, unreachable_code)]
mod renderer;
mod atom;
mod calculus;
mod file_manager;
mod resource;

use json;
use json::JsonValue;
use renderer::Typst;
use renderer::Renderer;
use std::process::Command;
use regex::Regex;
use atom::Atom;
use calculus::Calculus;
use std::fs::File;
use std::io::prelude::*;
use file_manager::{
    get_file_name,
    load_calculus,
    load_atoms,
    remove_extension,
    add_extension
};
use std::fs;

fn main() {
    let file_name = get_file_name()
        .unwrap_or("calculus1.json".to_string());
    let calculus = load_calculus(&file_name);
    let atoms = load_atoms(&calculus.get_members());
    let name = &remove_extension(&file_name);
    render_typ(calculus, &atoms, &name);
    render_pdf(&name);
}

fn render_pdf(name: &str) -> () {
    Command::new("quarto")
        .args(["typst", "compile", &format!("{}.typ", name)])
        .output()
        .expect("zut! ");
}


fn render_typ(calculus: Calculus, atoms: &[Atom], file_name: &str) -> () {
    let content = Typst(calculus, atoms.to_vec()).render();
    write_content(&add_extension(file_name, "typ"), &content);
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

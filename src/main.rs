#![allow(dead_code, unused_variables, unused_imports, unreachable_code)]
mod renderer;
mod atom;
mod calculus;

use std::env;
use json;
use json::JsonValue;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use renderer::Typst;
use renderer::Renderer;
use std::process::Command;
use regex::Regex;
use atom::Atom;
use calculus::Calculus;


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

fn get_file_name() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    Some(String::from(args.get(1)?.clone()))
}

fn load_atoms(names: &[String]) -> Vec<Atom> {
    let files = open_folder("atoms");
    files.iter()
        .map(|x| format!("atoms/{}", x))
        .map(|x| load(&x))
        .map(Atom::new)
        .filter(|x| names.iter().any(|y| y.clone() == x.name()))
        .map(Atom::check)
        .collect()
}

fn load_calculus(file_name: &String) -> Calculus {
    Calculus(load(&file_name)).check()
}

fn load(file_name: &String) -> JsonValue {
    let contents = fs::read_to_string(file_name)
                    .expect(&format!("le fichier '{}' est illisible", file_name));
    json::parse(&contents).expect("JSON Mal formé")
}

fn open_folder(chemin_dossier: &str) -> Vec<String> {
    let entries = fs::read_dir(chemin_dossier).expect("Dossier illisible");

    entries.into_iter().flatten()
        .map(|x| x.path())
        .filter(|x| x.is_file())
        .map(|x| x.file_name().unwrap().to_str().unwrap().to_string())
        .collect()
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

fn add_extension(s: &str, ext: &str) -> String {
    format!("{}.{}", s, ext)
}

fn remove_extension(file_name: &str) -> String {
    file_name.replace(".json", "")
}

pub fn json_to_array(j: &JsonValue) -> Vec<JsonValue> {
    if let JsonValue::Array(a) = j {
        a.clone()
    } else {
        panic!("{} should be an array", j)
    }
}

fn treat_variables(input: &str) -> String {
    let re = Regex::new(r"([a-zA-Z]+[0-9]+)").expect("Erreur lors de la création de l'expression régulière");
    let resultat = re.replace_all(input, "\"$1\"");
    resultat.to_string()
}

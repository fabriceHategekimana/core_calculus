use crate::{
    Atom,
    Calculus,
    JsonValue
};
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

pub fn get_file_name() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    Some(String::from(args.get(1)?.clone()))
}

pub fn load_atoms(names: &[String]) -> Vec<Atom> {
    let files = open_folder("atoms");
    files.iter()
        .map(|x| format!("atoms/{}", x))
        .map(|x| load(&x))
        .map(Atom::new)
        .filter(|x| names.iter().any(|y| y.clone() == x.name()))
        .map(Atom::check)
        .collect()
}

pub fn load_calculus(file_name: &String) -> Calculus {
    Calculus(load(&file_name)).check()
}

pub fn load(file_name: &String) -> JsonValue {
    let contents = fs::read_to_string(file_name)
                    .expect(&format!("le fichier '{}' est illisible", file_name));
    json::parse(&contents).expect("JSON Mal formé")
}

pub fn open_folder(chemin_dossier: &str) -> Vec<String> {
    let entries = fs::read_dir(chemin_dossier).expect("Dossier illisible");

    entries.into_iter().flatten()
        .map(|x| x.path())
        .filter(|x| x.is_file())
        .map(|x| x.file_name().unwrap().to_str().unwrap().to_string())
        .collect()
}

pub fn add_extension(s: &str, ext: &str) -> String {
    format!("{}.{}", s, ext)
}

pub fn remove_extension(file_name: &str) -> String {
    file_name.replace(".json", "")
}

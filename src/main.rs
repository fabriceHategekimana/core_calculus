#![allow(dead_code, unused_variables, unused_imports, unreachable_code)]
mod renderer;
mod atom;
mod calculus;
mod file_manager;
mod resource;

use json;
use json::JsonValue;
use renderer::{Typst, Renderer, render_typ};
use atom::{Atom, Atoms, AtomsT};
use calculus::Calculus;
use resource::Resource;
use file_manager::{
    get_file_name,
    load_atoms,
    remove_extension,
};

fn main() {
    let file_name = get_file_name()
        .unwrap_or("calculus1.json".to_string());
    let calculus = Calculus::load(&file_name);
    let atoms = Atoms::load("atoms/");
    render_typ(calculus, &atoms, &remove_extension(&file_name));
}

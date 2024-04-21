mod renderer;
mod atom;
mod calculus;
mod file_manager;
mod resource;

use std::fs;
use json::JsonValue;
use renderer::render_typ;
use atom::{Atom, Atoms, AtomsT};
use calculus::Calculus;
use resource::Resource;
use file_manager::{
    get_file_name,
    remove_extension,
};

use clap::{Command, Arg, ArgMatches};

fn get_user_input() -> ArgMatches {
    Command::new("MyApp")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("new")
                .about("Create a new project")
                .arg(Arg::new("name"))
        )
        .subcommand(
            Command::new("add")
                .about("Add a new axiom to the project")
                .arg(Arg::new("name"))
                   )
        .subcommand(
            Command::new("run")
                .about("Render the project")
                   )
        .get_matches()
}

fn new_project(s: Option<&str>) -> () {
    let name = s.expect("No project name given").to_string();
    fs::create_dir(name.clone()).expect("The folder can't be created");
    fs::create_dir(format!("{}/atoms", name)).expect("The folder can't be created");
    fs::create_dir(format!("{}/targets", name)).expect("The folder can't be created");
    fs::File::create(format!("{}/main.json", name)).expect("The file can't be created");
}

fn add_axiom(s: Option<&str>) -> () {
    let name = s.expect("No axiom name given").to_string();
    fs::File::create(format!("atoms/{}.json", name)).expect("The file can't be created");
}

fn run() -> () {
    let file_name = "main.json".to_string();
    let calculus = Calculus::load(&file_name);
    let atoms = Atoms::load("atoms/");
    render_typ(calculus, &atoms, &remove_extension(&file_name));
}

fn main() {
    match get_user_input().subcommand() {
        Some(("new", sub_matches)) => new_project(sub_matches.get_one::<String>("name").map(|x| x.as_str())),
        Some(("add", sub_matches)) => add_axiom(sub_matches.get_one::<String>("name").map(|x| x.as_str())),
        Some(("run", _)) => run(),
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}

// TODO
// quote formula
// quote rules

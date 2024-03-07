use std::env;
use std::fs;

pub fn get_file_name() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    Some(String::from(args.get(1)?.clone()))
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

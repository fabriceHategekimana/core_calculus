use std::fs;

pub fn get_file_name() -> Option<String> {
    let dossier_actuel = fs::read_dir(".").expect("We can't read the folder");

    dossier_actuel.into_iter()
        .flatten()
        .filter(|entry| entry.path().extension().is_some())
        .filter(|entry| entry.path().file_stem().is_some())
        .map(|x| x.path().to_string_lossy().into_owned())
        .nth(0)
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
    file_name.replace(".json", "").replace("./", "")
}

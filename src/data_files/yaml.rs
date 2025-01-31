use std::{fs, process};

use yaml_rust::{YamlLoader, Yaml};


pub struct YamlFile {
    pub file_path: String,
    pub content:   Vec<Yaml>,

} impl YamlFile {
    pub fn new(file_path: &str) -> YamlFile {
        if let Err(_) = fs::exists(file_path) {
            process::exit(1);
        };

        let file_content = &fs::read_to_string(file_path).unwrap();

        YamlFile { file_path: file_path.to_string(), content: YamlLoader::load_from_str(&file_content).unwrap() }
    }
}

use std::fs;

use linked_hash_map::LinkedHashMap;
use yaml_rust::{YamlLoader, YamlEmitter, Yaml};


pub struct YamlFile {
    pub file_path: String,
    pub content:   Yaml,

} impl YamlFile {
    pub fn new(file_path: &str) -> YamlFile {
        // First check if the file exists
        if !fs::exists(file_path).unwrap() {
            // Create a new Yaml map
            let file_yaml_content = Yaml::Hash(LinkedHashMap::new());

            // Convert the map into a string
            let mut file_string_content = String::new();
            let mut emitter = YamlEmitter::new(&mut file_string_content);
            emitter.dump(&file_yaml_content).unwrap();

            // Create the file with the strinified file content
            fs::write(file_path, file_string_content).unwrap();

            // Return the final file
            YamlFile { file_path: file_path.to_string(), content: file_yaml_content }
        } else {
            // Read the contents of the existing file into a Vec<Yaml>
            let file_content = fs::read_to_string(file_path).unwrap();
            println!("file_content: {}", file_content);
            let file_yaml_content: Vec<Yaml> = YamlLoader::load_from_str(&file_content).unwrap();

            // Return the final file using the first element in the Vector
            YamlFile { file_path: file_path.to_string(), content: file_yaml_content[0].clone() }
        }
    }

    pub fn update(&self) {
        // Convert the self.content variable into a string
        let mut file_content = String::new();
        let mut emitter = YamlEmitter::new(&mut file_content);
        emitter.dump(&self.content).unwrap();

        // Write this string to the saved file path
        fs::write(self.file_path.clone(), file_content)
            .expect("Failed to write to yaml file");
    }
}

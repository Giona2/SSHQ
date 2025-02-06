use crate::{data, stdin};
use crate::data_files::yaml::YamlFile;

use std::process;
use linked_hash_map::LinkedHashMap;
use yaml_rust::{YamlEmitter, YamlLoader, Yaml};


pub struct Commands;
impl Commands {
    pub fn help() {
        println!("sshq new <profile name>");
        println!("  create a new ssh profile to connect to");
        println!();
        println!("sshq connect <profile name>");
        println!("  connect to an existing ssh profile");
        println!();
        println!("sshq remove <profile name>");
        println!("  remove an existing profile");
        println!();
        println!("sshq list");
        println!("  list all saved ssh profiles");
    }

    pub fn new(profile_name: &str) {
        // getting the device/profile data
        let ip_addr      = stdin::input("IP Address        : ");
        let mut port_num = stdin::input("Port Number  [20] : ");
            if port_num == "" { port_num = String::from("20") };

        // create key pair files
        process::Command::new("ssh-keygen")
            .args(["-b", "4096"])
            .args(["-f", &(data::data_dir() + "/keys/" + &profile_name)])
            .args(["-N", "\"\""])
            //.stdout(process::Stdio::null())
            //.stderr(process::Stdio::null())
            .spawn()
            .expect("Failed to generate key");

        // create profile file
        let mut profile_file = YamlFile::new(&(data::data_dir() + "/profiles/" + &profile_name + ".yaml"));

        let mut profile_file_content: LinkedHashMap<Yaml, Yaml> = LinkedHashMap::new();
        profile_file_content.insert(Yaml::String("ip".to_string()),   Yaml::String(ip_addr));
        profile_file_content.insert(Yaml::String("port".to_string()), Yaml::String(port_num));

        profile_file.content = Yaml::Hash(profile_file_content);
        profile_file.update();
    }

    pub fn connect(profile_name: &str) {
        // get the username to connect to
        let target_username = stdin::input("Username: ");

        // get the profile data
        let profile_data_file = YamlFile::new(&(data::data_dir() + "/profiles/" + profile_name + ".yaml"));

        // connect via ssh
        process::Command::new("ssh")
            .args(["-p", profile_data_file.content["port"].as_str().unwrap()])
            .args(["-i", &(data::data_dir() + "/keys/" + profile_name)])
            .arg(&(target_username + "@" + profile_data_file.content["ip"].as_str().unwrap()))
            //.stdout(process::Stdio::null())
            //.stderr(process::Stdio::null())
            .spawn()
            .expect("Failed to connect");
    }

    pub fn remove(profile_name: &str) {
    }
    
    pub fn edit(profile_name: &str) {
    }

    pub fn list() {
    }
}

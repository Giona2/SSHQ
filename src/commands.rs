use crate::{data, stdin};
use crate::data_files::yaml::YamlFile;

use std::process;
use std::fs;
use linked_hash_map::LinkedHashMap;
use yaml_rust::Yaml;


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
        println!("sshq edit <profile name> <trait to change> <value to change to>");
        println!("                          |- ip  : change ip address");
        println!("                          |- port: change port number");
        println!();
        println!("  changes a selected value of an existing profile");
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
            .status()
            .expect("Failed to connect");
    }

    pub fn remove(profile_name: &str) {
        // remove profile file
        fs::remove_file(&(data::data_dir() + "/profiles/" + profile_name + ".yaml"))
            .expect("Failed to remove profile file");

        // remove private key
        fs::remove_file(&(data::data_dir() + "/keys/" + profile_name))
            .expect("Failed to remove profile file");

        // remove public key
        fs::remove_file(&(data::data_dir() + "/keys/" + profile_name + ".pub"))
            .expect("Failed to remove profile file");
    }
    
    pub fn edit(profile_name: &str, trait_to_change: &str, value_to_change_to: &str) {
        // get profile file
        let mut selected_profile_file = YamlFile::new(&(data::data_dir() + "/profiles/" + profile_name + ".yaml"));

        // change the selected trait
        if let Yaml::Hash(ref mut yaml_table) = selected_profile_file.content { match trait_to_change {
            "ip"   => { yaml_table.insert(Yaml::from_str("ip"),   Yaml::from_str(value_to_change_to)); }
            "port" => { yaml_table.insert(Yaml::from_str("port"), Yaml::from_str(value_to_change_to)); }
            _      => println!("This trait does not exist")
        }}

        // update the file and apply the changes
        selected_profile_file.update();
    }

    pub fn list() { for profile in fs::read_dir(&(data::data_dir() + "/profiles")).unwrap() {
        // get the file name of the current profile
        let profile_file_name =  profile.unwrap().file_name()
            .into_string().unwrap();
        
        // get the data retreaved from the profile file
        let profile_data_file = YamlFile::new(&(data::data_dir() + "/profiles/" + &profile_file_name));
        let profile_ip = profile_data_file.content["ip"].clone()
            .into_string().unwrap();
        let profile_port = profile_data_file.content["port"].clone()
            .into_string().unwrap();
        
        // get the profile's name by removing the .yaml file extension
        let profile_name = profile_file_name.replace(".yaml", "");

        // print the result
        println!("{profile_name} ({profile_ip}:{profile_port})");
   }}
}

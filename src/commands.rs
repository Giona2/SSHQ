use crate::stdin;

use ssh_rs::ssh;
use yaml_rust::YamlLoader;


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
        let ip_addr      = stdin::input("IP Address         : ");
        let port_num     = stdin::input("Port Number  [20]  : ");
        let key_password = stdin::input("Key password [None]: ");
    }

    pub fn connect(profile_name: &str) {
    }

    pub fn remove(profile_name: &str) {
    }
    
    pub fn edit(profile_name: &str) {
    }

    pub fn list() {
    }
}

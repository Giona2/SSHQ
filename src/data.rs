use whoami;

pub fn home_dir() -> String {
    format!("/home/{}", whoami::username().unwrap())
}

pub fn data_dir() -> String {
    format!("{}/.local/share/sshq", home_dir())
}

pub fn ssh_data_dir() -> String {
    format!("{}/.ssh", home_dir())
}

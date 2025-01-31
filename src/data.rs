use whoami;

pub fn data_dir() -> String {
    format!("/home/{}/.local/share/sshq", whoami::username().unwrap())
}

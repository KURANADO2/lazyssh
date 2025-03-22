use std::process::Command;

pub fn ssh_login(username: &str, host: &str, port: u32) {
    let ssh_cmd = format!("ssh {}@{} -p {}", username, host, port);
    println!("Executing: {}", ssh_cmd);

    Command::new("ssh")
        .arg(format!("-p {}", port))
        .arg(format!("{}@{}", username, host))
        .spawn()
        .expect("Failed to start SSH session")
        .wait()
        .expect("SSH process failed");
}

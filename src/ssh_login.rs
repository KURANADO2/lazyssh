use std::process::Command;

pub fn ssh_login(username: &str, hostname: &str, port: u32) {
    let ssh_cmd = format!("ssh {}@{} -p {}", username, hostname, port);
    println!("Executing: {}", ssh_cmd);

    Command::new("ssh")
        .arg(format!("-p {}", port))
        .arg(format!("{}@{}", username, hostname))
        .spawn()
        .expect("Failed to start SSH session")
        .wait()
        .expect("SSH process failed");
}

use crate::server::ServerItem;
use std::process::Command;

pub fn ssh_login(server: &ServerItem) {
    let ssh_cmd = format!(
        "ssh {}@{} -p {} -i {}",
        server.username, server.ip, server.port, server.private_key
    );
    println!("Executing: {}", ssh_cmd);

    Command::new("ssh")
        .arg(format!("{}@{}", server.username, server.ip))
        .arg("-p")
        .arg(&server.port.to_string())
        .arg("-i")
        .arg(&server.private_key)
        .spawn()
        .expect("Failed to start SSH session")
        .wait()
        .expect("SSH process failed");
}

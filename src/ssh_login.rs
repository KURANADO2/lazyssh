use crate::server::ServerItem;
use std::process::Command;

pub fn ssh_login(server: &ServerItem) {
    if let Some(password) = &server.password {
        // Use sshpass for password-based login
        let ssh_cmd = format!(
            "sshpass -p '{}' ssh {}@{} -p {}",
            "*".repeat(password.len()), server.username, server.ip, server.port
        );
        println!("Executing: {}", ssh_cmd);

        Command::new("sshpass")
            .arg("-p")
            .arg(password)
            .arg("ssh")
            .arg(format!("{}@{}", server.username, server.ip))
            .arg("-p")
            .arg(&server.port.to_string())
            .spawn()
            .expect("Failed to start SSH session with password")
            .wait()
            .expect("SSH process failed");
    } else {
        // Use key-based login
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
            .expect("Failed to start SSH session with key")
            .wait()
            .expect("SSH process failed");
    }
}

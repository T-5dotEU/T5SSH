use portable_pty::CommandBuilder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortForward {
    pub local_port: u16,
    pub remote_host: String,
    pub remote_port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshProfile {
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default)]
    pub user: Option<String>,
    #[serde(default)]
    pub identity_file: Option<String>,
    #[serde(default)]
    pub jump_host: Option<String>,
    #[serde(default)]
    pub port_forwards: Vec<PortForward>,
    #[serde(default)]
    pub agent_forwarding: bool,
}

fn default_port() -> u16 {
    22
}

pub fn build_ssh_command(profile: &SshProfile) -> CommandBuilder {
    let mut cmd = CommandBuilder::new("ssh");

    if let Some(ref user) = profile.user {
        cmd.arg("-l");
        cmd.arg(user);
    }

    if profile.port != 22 {
        cmd.arg("-p");
        cmd.arg(profile.port.to_string());
    }

    if let Some(ref identity) = profile.identity_file {
        cmd.arg("-i");
        cmd.arg(identity);
    }

    if let Some(ref jump) = profile.jump_host {
        cmd.arg("-J");
        cmd.arg(jump);
    }

    for fwd in &profile.port_forwards {
        cmd.arg("-L");
        cmd.arg(format!(
            "{}:{}:{}",
            fwd.local_port, fwd.remote_host, fwd.remote_port
        ));
    }

    if profile.agent_forwarding {
        cmd.arg("-A");
    }

    // Let SSH handle host key verification interactively in the terminal
    cmd.arg("-o");
    cmd.arg("StrictHostKeyChecking=ask");

    cmd.arg("-o");
    cmd.arg("ConnectTimeout=30");

    // Set TERM so remote applications (mc, htop, etc.) know the terminal
    // supports mouse tracking and 256 colors
    cmd.env("TERM", "xterm-256color");

    cmd.arg(&profile.host);

    cmd
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub ssh: SshProfile,
    #[serde(default = "default_rows")]
    pub rows: u16,
    #[serde(default = "default_cols")]
    pub cols: u16,
}

fn default_rows() -> u16 {
    24
}

fn default_cols() -> u16 {
    80
}

#[cfg(test)]
mod tests {
    use super::*;

    fn argv_contains(cmd: &CommandBuilder, needle: &str) -> bool {
        cmd.get_argv().iter().any(|a| a.to_str() == Some(needle))
    }

    #[test]
    fn test_basic_ssh_command() {
        let profile = SshProfile {
            host: "example.com".into(),
            port: 22,
            user: None,
            identity_file: None,
            jump_host: None,
            port_forwards: vec![],
            agent_forwarding: false,
        };
        let cmd = build_ssh_command(&profile);
        assert!(argv_contains(&cmd, "ssh"));
        assert!(argv_contains(&cmd, "example.com"));
        assert!(!argv_contains(&cmd, "-p"));
    }

    #[test]
    fn test_ssh_with_user_and_port() {
        let profile = SshProfile {
            host: "server.local".into(),
            port: 2222,
            user: Some("admin".into()),
            identity_file: None,
            jump_host: None,
            port_forwards: vec![],
            agent_forwarding: false,
        };
        let cmd = build_ssh_command(&profile);
        assert!(argv_contains(&cmd, "-l"));
        assert!(argv_contains(&cmd, "admin"));
        assert!(argv_contains(&cmd, "-p"));
        assert!(argv_contains(&cmd, "2222"));
    }

    #[test]
    fn test_ssh_with_identity_and_jump() {
        let profile = SshProfile {
            host: "target.com".into(),
            port: 22,
            user: Some("deploy".into()),
            identity_file: Some("/home/user/.ssh/id_ed25519".into()),
            jump_host: Some("bastion.com".into()),
            port_forwards: vec![],
            agent_forwarding: false,
        };
        let cmd = build_ssh_command(&profile);
        assert!(argv_contains(&cmd, "-i"));
        assert!(argv_contains(&cmd, "/home/user/.ssh/id_ed25519"));
        assert!(argv_contains(&cmd, "-J"));
        assert!(argv_contains(&cmd, "bastion.com"));
    }

    #[test]
    fn test_ssh_with_port_forward_and_agent() {
        let profile = SshProfile {
            host: "db-server.com".into(),
            port: 22,
            user: None,
            identity_file: None,
            jump_host: None,
            port_forwards: vec![
                PortForward {
                    local_port: 5432,
                    remote_host: "localhost".into(),
                    remote_port: 5432,
                },
                PortForward {
                    local_port: 8080,
                    remote_host: "internal.app".into(),
                    remote_port: 80,
                },
            ],
            agent_forwarding: true,
        };
        let cmd = build_ssh_command(&profile);
        assert!(argv_contains(&cmd, "-L"));
        assert!(argv_contains(&cmd, "5432:localhost:5432"));
        assert!(argv_contains(&cmd, "8080:internal.app:80"));
        assert!(argv_contains(&cmd, "-A"));
    }
}

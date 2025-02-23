use crate::container::Capabilities;

use super::{Container, ContainerError, ContainerRunRequest};
use std::process::Command;
pub struct Podman;

impl Podman {
    pub fn new() -> Self {
        Self
    }
}

impl Container for Podman {
    fn init(&self) -> Result<(), ContainerError> {
        // Initialize and start Podman machine
        // TODO: I think this may only be necessary on windows and mac?
        Command::new("podman")
            .args(["machine", "init"])
            .output()
            .map_err(|e| ContainerError::Init(e.to_string()))?;

        Command::new("podman")
            .args(["machine", "start"])
            .output()
            .map_err(|e| ContainerError::Init(e.to_string()))?;

        Ok(())
    }

    fn shell(&self, req: &ContainerRunRequest) -> Result<(), ContainerError> {
        let mut command = Command::new("podman");
        command
            .arg("run")
            .arg("-it")
            .arg("--rm")
            .arg("-v")
            .arg(format!("{}:/OUTPUT:Z", req.output_dir.display()))
            .arg("-w")
            .arg("/OUTPUT");

        for cap in &req.capabilities {
            command.arg("--cap-add").arg(cap.to_string());
        }

        command
            .arg("-e")
            .arg(r#"PS1=\[\033[1;32m\]podshot \[\033[0m\]:\[\033[1;34m\]\w\[\033[0m\]^ "#)
            .arg(&req.image)
            .arg("/bin/sh")
            .arg("-c")
            .arg(format!("{}exec /bin/sh", req.install_commands));

        command
            .status()
            .map_err(|e| ContainerError::Execution(e.to_string()))?;

        println!("Output directory: {}", req.output_dir.display());
        Ok(())
    }

    fn run(&self, req: &ContainerRunRequest, command: &str) -> Result<(), ContainerError> {
        let mut podman_command = Command::new("podman");
        podman_command
            .arg("run")
            .arg("--sig-proxy=true")
            .arg("-i")
            .arg("--rm")
            .arg("-v")
            .arg(format!("{}:/OUTPUT:Z", req.output_dir.display()))
            .arg("-w")
            .arg("/OUTPUT")
            .arg("-a")
            .arg("stdout")
            .arg("-a")
            .arg("stderr");

        for cap in &req.capabilities {
            podman_command.arg("--cap-add").arg(cap.to_string());
        }

        podman_command
            .arg(&req.image)
            .arg("/bin/sh")
            .arg("-c")
            .arg(format!("{}eval {}", req.install_commands, command));

        podman_command
            .status()
            .map_err(|e| ContainerError::Execution(e.to_string()))?;

        println!("Output directory: {}", req.output_dir.display());
        Ok(())
    }
}

//! Podman adapter
//!
//! This is a podman adapter for oneshot-- it contains code that is specific to running the oneshot
//! command using Podman.
use uuid::Uuid;

use super::{Container, ContainerError, ContainerRunRequest};
use std::{
    io::{BufRead, BufReader},
    path::PathBuf,
    process::{Command, Stdio},
};

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

    fn build_image(&self, container_file: &PathBuf) -> Result<String, ContainerError> {
        // Generate a unique image tag (you can tweak this)
        let image_tag = format!("oneshot-temp:{}", Uuid::new_v4());

        // The directory containing the Containerfile is the build context
        let build_context = container_file
            .parent()
            .ok_or_else(|| ContainerError::Init("Invalid Containerfile path".to_string()))?;

        // Prepare podman build command:
        // podman build -f <container_file> -t <image_tag> <build_context>
        let mut cmd = Command::new("podman");
        cmd.arg("build")
            .arg("-f")
            .arg(container_file)
            .arg("-t")
            .arg(&image_tag)
            .arg(build_context)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let mut child = cmd
            .spawn()
            .map_err(|e| ContainerError::Init(format!("Failed to spawn podman build: {}", e)))?;

        // Optionally, read and print output for debugging
        {
            let stdout = child.stdout.take().unwrap();
            let stderr = child.stderr.take().unwrap();

            let stdout_reader = BufReader::new(stdout);
            for line in stdout_reader.lines() {
                if let Ok(line) = line {
                    println!("[podman build stdout] {}", line);
                }
            }

            let stderr_reader = BufReader::new(stderr);
            for line in stderr_reader.lines() {
                if let Ok(line) = line {
                    eprintln!("[podman build stderr] {}", line);
                }
            }
        }

        let status = child
            .wait()
            .map_err(|e| ContainerError::Init(format!("Failed to wait on podman build: {}", e)))?;

        if !status.success() {
            return Err(ContainerError::Init(format!(
                "Podman build failed with status: {}",
                status
            )));
        }

        Ok(image_tag)
    }

    fn shell(&self, req: &ContainerRunRequest) -> Result<(), ContainerError> {
        let image_tag = self.build_image(&req.container_file)?;

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
            .arg(&image_tag)
            .arg("/bin/sh")
            .arg("-c")
            .arg(format!("{} exec /bin/sh", req.commands.to_string()));

        command
            .status()
            .map_err(|e| ContainerError::Execution(e.to_string()))?;

        println!("Output directory: {}", req.output_dir.display());
        Ok(())
    }

    fn run(&self, req: &ContainerRunRequest) -> Result<(), ContainerError> {
        // Build the image from the Containerfile path first
        let image_tag = self.build_image(&req.container_file)?;

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
            .arg(&image_tag)
            .arg("/bin/sh")
            .arg("-c")
            .arg(format!("{}", req.commands.to_string()));

        podman_command
            .status()
            .map_err(|e| ContainerError::Execution(e.to_string()))?;

        println!("Output directory: {}", req.output_dir.display());
        Ok(())
    }
}

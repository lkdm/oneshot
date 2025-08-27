pub mod podman;
use std::fmt;
use std::path::PathBuf;
use std::process::Command;

use clap::ValueEnum;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ContainerError {
    #[error("could not init container")]
    Init(String),
    #[error("error while executing")]
    Execution(String),
}

#[derive(Clone, Debug, ValueEnum)]
pub enum Capabilities {
    NetRaw,
    NetAdmin,
}

impl fmt::Display for Capabilities {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Capabilities::NetRaw => write!(f, "NET_RAW"),
            Capabilities::NetAdmin => write!(f, "NET_ADMIN"),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Commands(pub Vec<String>);

impl FromIterator<String> for Commands {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Commands(iter.into_iter().collect())
    }
}

impl std::fmt::Display for Commands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // join all args as a shell-escaped single command string
        let joined = shell_words::join(&self.0);
        write!(f, "{}", joined)
    }
}

pub struct ContainerRunRequest {
    image: String,
    output_dir: PathBuf,
    capabilities: Vec<Capabilities>,
    commands: Commands,
}

impl ContainerRunRequest {
    pub fn new(
        image: &str,
        output_dir: PathBuf,
        capabilities: Vec<Capabilities>,
        commands: Commands,
    ) -> Self {
        Self {
            image: image.to_string(),
            output_dir: output_dir.into(),
            capabilities,
            commands,
        }
    }
}

/// Container
///
/// A common interface for container adapters.
pub trait Container {
    fn init(&self) -> Result<(), ContainerError>;
    fn shell(&self, req: &ContainerRunRequest) -> Result<(), ContainerError>;
    fn run(&self, req: &ContainerRunRequest) -> Result<(), ContainerError>;
}

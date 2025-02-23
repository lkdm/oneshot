pub mod podman;
use std::path::PathBuf;
use std::{env, fmt};

use clap::ValueEnum;
use thiserror::Error;

#[derive(Debug, Clone)]
struct ContainerImage {
    name: String,
    url: String,
    tags: Vec<String>,
}
impl ContainerImage {
    pub fn new(name: &str, url: &str, tags: &str) -> Self {
        Self {
            name: name.into(),
            tags: tags.split(',').map(|s| s.trim().to_string()).collect(),
            url: url.into(),
        }
    }
}

struct ContainerImages(Vec<ContainerImage>);
impl Default for ContainerImages {
    fn default() -> Self {
        Self(
            [
                ContainerImage::new("rust", "docker.io/library/rust:alpine", "rust,cargo,rustup"),
                ContainerImage::new(
                    "bun",
                    "docker.io/oven/bun:alpine",
                    "bun,js,ts,bunsh,typescript",
                ),
                ContainerImage::new("uv", "ghcr.io/astral-sh/uv:alpine", "uv,python,python3,pip"),
            ]
            .to_vec(),
        )
    }
}

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

pub struct ContainerRunRequest {
    image: String,
    output_dir: PathBuf,
    capabilities: Vec<Capabilities>,
    install_commands: InstallCommand,
}

impl ContainerRunRequest {
    pub fn new(
        image: &str,
        output_dir: PathBuf,
        capabilities: Vec<Capabilities>,
        install_commands: &InstallCommand,
    ) -> Self {
        Self {
            image: image.to_string(),
            output_dir: output_dir.into(),
            capabilities: capabilities,
            install_commands: install_commands.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct InstallCommand(String);

impl fmt::Display for InstallCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug)]
pub struct InstallCommandBuilder(Vec<String>);

impl InstallCommandBuilder {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn with_apk(&mut self, packages: Option<&[String]>) -> &mut Self {
        let Some(packages) = packages else {
            return self;
        };
        self.0.push("apk update".into());
        if !packages.is_empty() {
            let cmd = format!("apk add --no-cache {}", packages.join(" "));
            self.0.push(cmd);
        }
        self
    }

    pub fn with_cargo(&mut self, packages: Option<&[String]>) -> &mut Self {
        let Some(packages) = packages else {
            return self;
        };
        self.0.push("apk add --no-cache cargo".into());
        if !packages.is_empty() {
            let cmd = format!("cargo install {}", packages.join(" "));
            self.0.push(cmd);
        }
        self
    }

    pub fn with_uv(&mut self, packages: Option<&[String]>) -> &mut Self {
        let Some(packages) = packages else {
            return self;
        };
        self.0.extend([
            "apk add --no-cache python3 py3-pip".into(),
            "python3 -m venv /app/venv".into(),
            ". /app/venv/bin/activate".into(),
            "pip install uv".into(),
        ]);
        if !packages.is_empty() {
            let cmd = format!("uv pip install {}", packages.join(" "));
            self.0.push(cmd);
        }
        self.0
            .extend(["deactivate".into(), ". /app/venv/bin/activate".into()]);
        self
    }

    pub fn with_bun(&mut self, packages: Option<&[String]>) -> &mut Self {
        let Some(packages) = packages else {
            return self;
        };
        self.0.extend([
            "apk add --no-cache curl bash".into(),
            "curl -fsSL https://bun.sh/install | bash".into(),
            "export PATH=/root/.bun/bin:$PATH".into(),
        ]);
        if !packages.is_empty() {
            let cmd = format!("bun install {}", packages.join(" "));
            self.0.push(cmd);
        }
        self
    }

    pub fn with_git(&mut self, repos: Option<&[String]>) -> &mut Self {
        let Some(repos) = repos else {
            return self;
        };
        self.0.push("apk add --no-cache git".into());
        for repo in repos {
            let cmd = format!("git clone {}", repo);
            self.0.push(cmd);
        }
        self
    }

    pub fn with_rubygems(&mut self, packages: Option<&[String]>) -> &mut Self {
        let Some(packages) = packages else {
            return self;
        };
        self.0.push("apk add --no-cache ruby".into());
        if !packages.is_empty() {
            let cmd = format!("gem install {}", packages.join(" "));
            self.0.push(cmd);
        }
        self
    }

    pub fn with_npm(&mut self, packages: Option<&[String]>) -> &mut Self {
        let Some(packages) = packages else {
            return self;
        };
        self.0.push("apk add --no-cache nodejs npm".into());
        if !packages.is_empty() {
            let cmd = format!("npm install -g {}", packages.join(" "));
            self.0.push(cmd);
        }
        self
    }

    pub fn with_pip(&mut self, packages: Option<&[String]>) -> &mut Self {
        let Some(packages) = packages else {
            return self;
        };
        self.0.push("apk add --no-cache python3 py3-pip".into());
        if !packages.is_empty() {
            let cmd = format!("pip install {}", packages.join(" "));
            self.0.push(cmd);
        }
        self
    }

    pub fn with_flatpak(&mut self, packages: Option<&[String]>) -> &mut Self {
        let Some(packages) = packages else {
            return self;
        };
        self.0.push("apk add --no-cache flatpak".into());
        self.0.push("flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo".into());
        for package in packages {
            let cmd = format!("flatpak install -y flathub {}", package);
            self.0.push(cmd);
        }
        self
    }

    pub fn build(&mut self) -> InstallCommand {
        InstallCommand(self.0.join(" && "))
    }
}

/// Container
///
/// A common interface for container adapters.
pub trait Container {
    fn init(&self) -> Result<(), ContainerError>;
    fn shell(&self, req: &ContainerRunRequest) -> Result<(), ContainerError>;
    fn run(&self, req: &ContainerRunRequest, command: &str) -> Result<(), ContainerError>;
}

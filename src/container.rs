use std::path::PathBuf;

use clap::ValueEnum;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ContainerError {
    #[error("could not init container")]
    Init,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum Capabilities {
    NetRaw,
    NetAdmin,
}

pub struct ContainerRunRequest {
    image: String,
    output_dir: PathBuf,
    capabilities: Vec<Capabilities>,
    install_commands: InstallCommand,
}

pub struct InstallCommand(String);

pub struct InstallCommandBuilder(Vec<String>);

impl InstallCommandBuilder {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn with_apk(&mut self, packages: &[String]) {
        self.0.push("apk update".into());
        if !packages.is_empty() {
            let cmd = format!("apk add --no-cache {}", packages.join(" "));
            self.0.push(cmd);
        }
    }

    pub fn with_cargo(&mut self, packages: &[String]) {
        self.0.push("apk add --no-cache cargo".into());
        if !packages.is_empty() {
            let cmd = format!("cargo install {}", packages.join(" "));
            self.0.push(cmd);
        }
    }

    pub fn with_uv(&mut self, packages: &[String]) {
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
    }

    pub fn with_bun(&mut self, packages: &[String]) {
        self.0.extend([
            "apk add --no-cache curl bash".into(),
            "curl -fsSL https://bun.sh/install | bash".into(),
            "export PATH=/root/.bun/bin:$PATH".into(),
        ]);
        if !packages.is_empty() {
            let cmd = format!("bun install {}", packages.join(" "));
            self.0.push(cmd);
        }
    }

    pub fn with_git(&mut self, repos: &[String]) {
        self.0.push("apk add --no-cache git".into());
        for repo in repos {
            let cmd = format!("git clone {}", repo);
            self.0.push(cmd);
        }
    }

    pub fn with_rubygems(&mut self, packages: &[String]) {
        self.0.push("apk add --no-cache ruby".into());
        if !packages.is_empty() {
            let cmd = format!("gem install {}", packages.join(" "));
            self.0.push(cmd);
        }
    }

    pub fn with_npm(&mut self, packages: &[String]) {
        self.0.push("apk add --no-cache nodejs npm".into());
        if !packages.is_empty() {
            let cmd = format!("npm install -g {}", packages.join(" "));
            self.0.push(cmd);
        }
    }

    pub fn with_pip(&mut self, packages: &[String]) {
        self.0.push("apk add --no-cache python3 py3-pip".into());
        if !packages.is_empty() {
            let cmd = format!("pip install {}", packages.join(" "));
            self.0.push(cmd);
        }
    }

    pub fn with_flatpak(&mut self, packages: &[String]) {
        self.0.push("apk add --no-cache flatpak".into());
        self.0.push("flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo".into());
        for package in packages {
            let cmd = format!("flatpak install -y flathub {}", package);
            self.0.push(cmd);
        }
    }

    pub fn build(self) -> InstallCommand {
        InstallCommand(self.0.join(" && "))
    }
}

/// Container
///
/// A common interface for container adapters.
pub trait Container {
    fn init() -> Result<(), ContainerError>;
    fn shell(req: &ContainerRunRequest) -> Result<(), ContainerError>;
    fn run(req: &ContainerRunRequest, command: &str) -> Result<(), ContainerError>;
}

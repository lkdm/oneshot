use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "oneshot")]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Run(RunArgs),
}

#[derive(Parser, Debug)]
pub struct RunArgs {
    /// Path to tool directory (contains Containerfile)
    #[arg()]
    pub tool: String,

    /// Commands to run in the container, separated by `--`
    #[arg(trailing_var_arg = true)]
    pub commands: Vec<String>,
}


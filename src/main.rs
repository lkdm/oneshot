use std::env;

use clap::Parser;
use oneshot::{
    cli::{Cli, Commands},
    container::{Container, ContainerRunRequest, podman::Podman},
};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run(run_args) => {
            println!("Tool path: {}", run_args.tool);

            // Convert Vec<String> into Commands newtype
            let commands = oneshot::container::Commands(run_args.commands);

            println!("Commands to run: {}", commands);

            // Example: build container request
            let output_dir = env::current_dir().expect("Failed to get cwd");

            let req = ContainerRunRequest::new(
                &run_args.tool, // assuming this is the container image or tool name
                output_dir,
                vec![], // capabilities (empty for now)
                commands,
            );

            // Instantiate Podman container runner
            let container = Podman::new();

            // Run container with commands
            if let Err(e) = container.run(&req) {
                eprintln!("Container run failed: {}", e);
            }
        }
    }
}

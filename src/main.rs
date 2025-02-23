use clap::{Parser, Subcommand, ValueEnum};
use oneshot::container::{
    Capabilities, Container, ContainerRunRequest, InstallCommandBuilder, podman::Podman,
};
use std::{env, path::PathBuf};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long, help = "Container image", default_value = &"alpine:latest")]
    image: String,

    #[arg(
        short,
        long,
        help = "Host output directory",
        long_help = "Host directory where the output will be stored. If not specified, output will be the current directory"
    )]
    output_dir: Option<PathBuf>,

    #[arg(short, long, value_enum, num_args = 1..,
		help = "Elevate container privileges",
		long_help = "Add specific privileges to the container. This can increase the container's capabilities within the host system."
	)]
    cap_add: Option<Vec<Capabilities>>,

    #[arg(long, num_args = 1..,
    	help = "Use APK packages",
    	long_help = "Specify packages to be installed using APK (Alpine Package Keeper). Multiple packages can be listed."
	)]
    from_apk: Option<Vec<String>>,

    #[arg(long, num_args = 0..,
    	help = "Use Git repositories",
    	long_help = "Pull repositories from Git. If no arguments are provided, it will just install Git."
	)]
    from_git: Option<Vec<String>>,

    #[arg(long, num_args = 0..,
    	help = "Use Cargo packages",
		long_help = "Install packages using Cargo (Rust's package manager). If no arguments are provided, it will just install Cargo. Installs Cargo's dependencies like rustup."
    )]
    from_cargo: Option<Vec<String>>,

    #[arg(long, num_args = 0..,
    	help = "Use UV packages",
		long_help = "Install packages using UV. If no arguments are provided, it will just install UV."
    )]
    from_uv: Option<Vec<String>>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Run a shell command within the oneshot container")]
    Run {
        #[arg(
            short,
            long,
            help = "Shell command to run",
            long_help = "The string will be evaluated as a Bash script within the container."
        )]
        script: String,
    },
    #[command(about = "Run an interactive shell within the oneshot container")]
    Shell,
    #[command(about = "Execute a given script within the oneshot container")]
    Exec {
        #[arg(short, long)]
        path: Option<std::path::PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();
    let install_commands = InstallCommandBuilder::new()
        .with_apk(cli.from_apk.as_deref())
        .with_git(cli.from_git.as_deref())
        .with_cargo(cli.from_cargo.as_deref())
        .with_uv(cli.from_uv.as_deref())
        .build();

    let req = ContainerRunRequest::new(cli.image, cli.output_dir, cli.cap_add, &install_commands);

    let container: Podman = Podman::new();

    // Now you can use container_run_request to run your container
    match &cli.command {
        Some(Commands::Run { script }) => {
            // Run the script in the container
            container.init();
            container.run(&req, script);
        }
        Some(Commands::Shell) => {
            // Start an interactive shell in the container
            container.init();
            container.shell(&req);
        }
        Some(Commands::Exec { path }) => {
            // Execute the script at the given path in the container
        }
        None => {
            // Handle case when no subcommand is provided
        }
    }
}

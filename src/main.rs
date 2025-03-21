use clap::{Args, Parser, Subcommand, ValueEnum};
use oneshot::container::{
    Capabilities, Container, ContainerRunRequest, InstallCommandBuilder, podman::Podman,
};
use std::{env, path::PathBuf};

#[derive(Args, Clone, Debug)]
struct CommonArgs {
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

#[derive(Parser)]
#[command(author, version, about, long_about = None,
	after_help=r#"REPLS:
  Python       oneshot run -s "python3 -i" --from-uv numpy
  Typescript   oneshot run -s "bun repl" --from-bun
"#
	)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[clap(flatten)]
    common: CommonArgs,
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
        #[clap(flatten)]
        common: CommonArgs,
    },
    #[command(about = "Run an interactive shell within the oneshot container")]
    Shell {
        #[clap(flatten)]
        common: CommonArgs,
    },
 }

fn main() {
    let cli = Cli::parse();
    let container: Podman = Podman::new();

    // Now you can use container_run_request to run your container
    match &cli.command {
        Commands::Run { script, common } => {
            let install_commands = InstallCommandBuilder::new()
                .with_apk(common.from_apk.as_deref())
                .with_git(common.from_git.as_deref())
                .with_cargo(common.from_cargo.as_deref())
                .with_uv(common.from_uv.as_deref())
                .build();

            let output_dir = common
                .output_dir
                .clone()
                .unwrap_or_else(|| env::current_dir().expect("Failed to get current directory"));

            let req = ContainerRunRequest::new(
                &common.image,
                output_dir,
                common.cap_add.clone().unwrap_or_default(),
                &install_commands,
            );

            container.init();
            container.run(&req, script);
        }
        Commands::Shell { common } => {
            // Start an interactive shell in the container
            let install_commands = InstallCommandBuilder::new()
                .with_apk(common.from_apk.as_deref())
                .with_git(common.from_git.as_deref())
                .with_cargo(common.from_cargo.as_deref())
                .with_uv(common.from_uv.as_deref())
                .build();

            let output_dir = common
                .output_dir
                .clone()
                .unwrap_or_else(|| env::current_dir().expect("Failed to get current directory"));

            let req = ContainerRunRequest::new(
                &common.image,
                output_dir,
                common.cap_add.clone().unwrap_or_default(),
                &install_commands,
            );

            container.init();
            container.shell(&req);
        }
    }
}

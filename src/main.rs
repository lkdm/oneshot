use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None)]
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
    cap_add: Option<Vec<Privileges>>,

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

#[derive(Clone, Debug, ValueEnum)]
enum Privileges {
    NetRaw,
    NetAdmin,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        #[arg(short, long)]
        script: String,
    },
    Shell,
    Exec {
        #[arg(short, long)]
        path: std::path::PathBuf,
    },
}

//   -i, --image <image>  Specify a target image, default=alpine:latest
//   -o, --output-dir <path>    Specify the output directory (default: current directory)
//   -c, --cap-add [privs]       List of priviliges to add
//
// Install packages:
//   --from-apk   [pkgs]  Install packages from apk
//   --from-git   [repos] Download repositories from git
//   --from-cargo [pkgs]  Install packages from cargo
//   --from-uv    [pkgs]  Install packages from uv

fn main() {
    let cli = Cli::parse();
}

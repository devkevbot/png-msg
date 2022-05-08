use clap::Parser;
use std::path::PathBuf;

pub fn run() -> Args {
    Args::parse()
}

#[derive(Debug, clap::Parser)]
pub struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, clap::Subcommand)]
enum Command {
    Encode {
        #[clap(required = true, parse(from_os_str))]
        input_path: PathBuf,

        #[clap(required = true)]
        chunk_type: String,

        #[clap(required = true)]
        message: String,

        #[clap(parse(from_os_str))]
        output_path: Option<PathBuf>,
    },

    Decode {
        #[clap(required = true, parse(from_os_str))]
        input_path: PathBuf,

        #[clap(required = true)]
        chunk_type: String,
    },

    Remove {
        #[clap(required = true, parse(from_os_str))]
        input_path: PathBuf,

        #[clap(required = true)]
        chunk_type: String,
    },

    Print {
        #[clap(required = true, parse(from_os_str))]
        input_path: PathBuf,
    },
}

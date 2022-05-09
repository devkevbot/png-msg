use crate::commands;
use crate::Result;
use clap::Parser;
use std::path::PathBuf;

pub fn run() -> Result<()> {
    execute(Args::parse())
}

fn execute(args: Args) -> Result<()> {
    match args.command {
        Command::Encode {
            input_path,
            chunk_type,
            message,
            output_path,
        } => commands::encode(input_path, chunk_type, message, output_path),

        Command::Decode {
            input_path,
            chunk_type,
        } => commands::decode(input_path, chunk_type),

        Command::Remove {
            input_path,
            chunk_type,
        } => commands::remove(input_path, chunk_type),

        Command::Print { input_path } => commands::print(input_path),
    }
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

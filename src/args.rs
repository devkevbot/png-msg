use crate::commands;
use crate::{Error, Result};
use clap::Parser;
use std::path::PathBuf;

pub fn run() -> Result<()> {
    let args = Args::parse();
    let output = execute(args)?;
    Ok(output)
}

fn execute(args: Args) -> Result<()> {
    match args.command {
        Command::Encode {
            input_path,
            chunk_type,
            message,
            output_path,
        } => {
            println!(
                "{:?}, {}, {}, {:?}",
                input_path, chunk_type, message, output_path
            );
            let res = commands::encode(input_path, chunk_type, message, output_path)?;
            Ok(res)
        }
        Command::Decode {
            input_path,
            chunk_type,
        } => {
            println!("{:?}, {}", input_path, chunk_type);
            commands::decode(input_path, chunk_type)?;
            Ok(())
        }
        Command::Remove {
            input_path,
            chunk_type,
        } => {
            println!("{:?}, {}", input_path, chunk_type);
            commands::remove(input_path, chunk_type)?;
            Ok(())
        }
        Command::Print { input_path } => {
            println!("{:?}", input_path);
            commands::print(input_path)?;
            Ok(())
        }
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

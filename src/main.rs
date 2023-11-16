#![warn(clippy::pedantic, clippy::nursery)]
use std::error::Error;

use clap::{Parser, Subcommand};

use ascii_grep::{CharEntry, CharFile};

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    sub_command: SubCommand,
}

#[derive(Subcommand)]
enum SubCommand {
    Find { search: String },
    List,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let index: CharFile = bincode::deserialize(include_bytes!("../index"))?;
    match args.sub_command {
        SubCommand::Find { search } => {
            for CharEntry { char, description } in index.rows {
                if !description.contains(&search) {
                    continue;
                }
                println!("{char:?} {description}");
            }
        }
        SubCommand::List => {
            for CharEntry { char, description } in index.rows {
                println!("{char:?} {description}");
            }
        }
    }
    Ok(())
}

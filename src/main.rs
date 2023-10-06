use std::{str::FromStr, path::PathBuf};
use args::PngMeArgs;
use clap::Parser;
use args::Commands;
use commands::{encode, decode, remove, print};

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;


pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let png_me_args = PngMeArgs::parse();
    
    match png_me_args.command {
        Commands::Encode(args) => {
            encode(&args);
        }
        Commands::Decode(args) => {
           decode(&args);
        }
        Commands::Remove(args) => {
           remove(&args);
        }
        Commands::Print(args) => {
          print(&args);
        }
    }

    Ok(())
}
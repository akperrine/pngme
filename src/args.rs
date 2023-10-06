use std::{fs::File, path::PathBuf};

use clap:: {Args, Parser, Subcommand};


#[derive(Parser, Debug)]
pub struct PngMeArgs {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Encode the message in the specfic PNG file with a  certian type
    Encode(EncodeArgs),
    /// Decode the message in the specfic PNG file according to a certian chunk type
    Decode(DecodeArgs),
    /// Remove a message according to certian chunk type
    Remove(RemoveArgs),
    /// Print a list of PNG chunks that can be searched for messages
    Print(PrintArgs),
}

#[derive(Debug, Args, Clone)]
pub struct EncodeArgs {
    /// PNG file path
    pub file_path: PathBuf,
    /// Chunk Type
    pub chunk_type: String,
    /// Secret message
    pub message: String,
    /// Optional file output path
    pub output_file: Option<PathBuf>
}

#[derive(Debug, Args, Clone)]
pub struct DecodeArgs {
     /// PNG file path
     pub file_path: PathBuf,
     /// Chunk Type
     pub chunk_type: String,
}

#[derive(Debug, Args, Clone)]
pub struct RemoveArgs {
     /// Input PNG file path
     pub file_path: PathBuf,
     /// Chunk Type
     pub chunk_type: String,
}

#[derive(Debug, Args, Clone)]
pub struct PrintArgs {
    /// Input PNG file path
    pub file_path: PathBuf,
}
use std::str::FromStr;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;


pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let chunk = chunk_type::ChunkType::from_str("1ust");
    println!("{}",chunk.is_err());
    todo!();
}
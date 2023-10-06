use crate::Error;
use crate::args::{EncodeArgs, DecodeArgs, RemoveArgs, PrintArgs};
use crate::chunk::Chunk;
use crate::chunk_type::{self, ChunkType};
use crate::png::{Png};
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use std::io;


pub fn encode(args: &EncodeArgs) -> Result<(), Error> {
    // println!("{:?}", args);
    let png_bytes = fs::read(&args.file_path)?;
    // println!("png bytes {:?}", png_bytes);
    let mut png = Png::try_from(png_bytes.as_slice()).unwrap();
    let chunk_type = ChunkType::from_str(&args.chunk_type).unwrap();
    let message_as_bytes = args.message.as_bytes().to_vec();
    let chunk = Chunk::new(chunk_type, message_as_bytes);
    
    png.append_chunk(chunk);

    if let Some(output_file) = &args.output_file {
        fs::write(output_file, png.as_bytes());
    } else {
        fs::write(&args.file_path, png.as_bytes());
    }
    println!("{}", png);
    
    Ok(())
}

pub fn decode(args: &DecodeArgs) -> Result<(), Error> {
    println!("run decode");
    let png_bytes = fs::read(&args.file_path)?;
    // // println!("png bytes {:?}", png_bytes);
    let png = Png::try_from(png_bytes.as_slice()).unwrap();

    if let Some(chunk) = png.chunk_by_type(&args.chunk_type) {
        // println!("message");
        println!("msg: {}", chunk.data_as_string().unwrap());
        return Ok(())
    } else {
        println!("didn't hit");
       return Err(io::Error::new(
        io::ErrorKind::NotFound,
        "Chunk with specified type not found",
    )
    .into());
    }
    Ok(())
}

pub fn remove(args: &RemoveArgs) -> Result<(), Error> {

    println!("run remove");
    let png_bytes = fs::read(&args.file_path)?;
    let mut png = Png::try_from(png_bytes.as_slice()).unwrap();
    png.remove_chunk(&args.chunk_type)?;
    
    fs::write(&args.file_path, png.as_bytes());

    Ok(())
}

pub fn print(args: &PrintArgs) -> Result<(), Error> {
    let png_bytes = fs::read(&args.file_path)?;
    let png = Png::try_from(png_bytes.as_slice()).unwrap();

    println!("File: {:?}", &args.file_path);

    for (i, chunk) in png.chunks().iter().enumerate() {
        println!(
            "  chunk#{}{{ chunk_type: {}, data_length: {}}}",
            i,
            chunk.chunk_type(),
            chunk.length(),
        );
    }

    Ok(())
}

pub fn create_png_struct(file_path: PathBuf) -> Result<Png, Error> {
    let png_bytes = fs::read(file_path)?;
     Ok(Png::try_from(png_bytes.as_slice()).unwrap())
}
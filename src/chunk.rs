use crate::{chunk_type::ChunkType, Error};
use std::str::FromStr;
use std::fmt;
use crc::{Crc, CRC_32_ISO_HDLC};

#[derive(Debug)]
pub struct Chunk {
    length: u32,
    pub chunk_type: ChunkType,
    chunk_data: Vec<u8>,
    crc: u32
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}

impl TryFrom<&Vec<u8>> for Chunk{
    type Error = &'static str;

    fn try_from(bytes: &Vec<u8>) -> Result<Self, Self::Error> {
        if bytes.len() < 12 {
            return Err("Not enough bytes for a valid Chunk")
        }

        let length_bytes: [u8; 4] = [bytes[0], bytes[1], bytes[2], bytes[3]];
        let length = u32::from_be_bytes(length_bytes);
        // println!("length: {}", length);
        let chunk_type_bytes: [u8; 4] = [bytes[4], bytes[5], bytes[6], bytes[7]];
        let chunk_type = ChunkType::try_from(chunk_type_bytes).unwrap();
        // println!("chunk type: {}", chunk_type);

        let chunk_data = bytes[8..bytes.len() - 4].to_vec();
        // println!("chunk data arr: {:?}", chunk_data);

        let crc_bytes: [u8; 4] = [
            bytes[bytes.len() - 4],
            bytes[bytes.len() - 3],
            bytes[bytes.len() - 2],
            bytes[bytes.len() - 1],
        ];
        // println!("crc arr: {:?}", crc_bytes);
        let crc = u32::from_be_bytes(crc_bytes);
        // println!("crc {}", crc);

        let to_check = [&chunk_type.bytes(), chunk_data.as_slice()].concat();
        // println!("expected crc {}", Chunk::calc_checksum(&to_check));
        // println!("compare {} to {}",Chunk::calc_checksum(&to_check), crc);
        if Chunk::calc_checksum(&to_check) != crc {
            return Err("Data is corrupted")
        }

        Ok(Chunk { length, chunk_type, chunk_data, crc })
    }
}


impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let to_check = [&chunk_type.bytes(), data.as_slice()].concat();
        let crc = Chunk::calc_checksum(&to_check);

        
        let chunk = Chunk {
            length: data.len().try_into().unwrap(),
            chunk_type,
            chunk_data: data,
            crc,
        };

        chunk
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.chunk_data
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String, Error> {
        let chunk_message:String = String::from_utf8(self.chunk_data.to_vec()).expect("Invalid UTF-8");

        Ok(chunk_message)
        
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.length.to_be_bytes());
        bytes.extend_from_slice(&self.chunk_type.bytes());
        bytes.extend_from_slice(&self.chunk_data);
        bytes.extend_from_slice(&self.crc.to_be_bytes());

        bytes
    }

    fn calc_checksum(bytes: &[u8]) -> u32 {

        let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC);
       
        crc.checksum(bytes)
    }

}

pub fn chunk_from_strings(chunk_type_input: &str, message: &str) -> Result<Chunk, Error> {
    let chunk_type = ChunkType::from_str(chunk_type_input).unwrap();
     let chunk = Chunk::new(chunk_type, message.as_bytes().to_vec());

     Ok(chunk)
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!".as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        
        let _chunk_string = format!("{}", chunk);
    }
}

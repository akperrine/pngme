use crate::Error;
use std::fmt;
use std::io;
use std::str::FromStr;
use regex::Regex;


#[derive(Debug, PartialEq)]
pub struct ChunkType {
    ancillary: u8,
    private: u8,
    reserved: u8,
    safe_to_copy: u8
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ();

    fn try_from(byte_array: [u8; 4]) -> Result<Self, Self::Error> {

         Ok(ChunkType { ancillary: byte_array[0], private: byte_array[1], reserved: byte_array[2], safe_to_copy: byte_array[3] })
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         let bytes: [u8; 4] = [self.ancillary, self.private, self.reserved, self.safe_to_copy];
        let ascii_string = String::from_utf8(bytes.to_vec()).expect("Invalid UTF-8");

        write!(f,"{}", ascii_string)
    }

}

impl FromStr for ChunkType {
    type Err = Error;
   
        fn from_str(input: &str) -> Result<ChunkType, Error> {

        if input.len() != 4 {
            return Err(Box::new(io::Error::new(io::ErrorKind::InvalidInput, "Input string slice must have 4 bytes",)))
        }

        let letters = Regex::new(r"^[a-zA-Z]+$").unwrap();
        
        if !letters.is_match(input) {
            return Err(Box::new(io::Error::new(io::ErrorKind::InvalidInput, "Must be a ASCII character A-Z or a-z",)))
        }
        
        

        let ancillary = input.as_bytes()[0];
        let private = input.as_bytes()[1];
        let reserved = input.as_bytes()[2];
        let safe_to_copy = input.as_bytes()[3];

        let result = ChunkType {
            ancillary,
            private,
            reserved,
            safe_to_copy,
        };

        Ok(result)
    }
    
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        [self.ancillary, self.private, self.reserved, self.safe_to_copy]
    }

    pub fn is_valid(&self) -> bool {

         if self.ancillary.is_ascii() &&
         self.private.is_ascii() &&
         self.reserved.is_ascii() &&
         self.safe_to_copy.is_ascii(){
            return true
         } else {
            return false
         }
    }

    pub fn is_critical(&self) -> bool {
        let byte = self.ancillary;

        // 0 represents critial
        return !check_fifth_bit(&byte);
        
    }

    pub fn is_public(&self) -> bool {
        let byte = self.private;

        // 0 represents public
        return !check_fifth_bit(&byte);
        
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        let byte = self.reserved;

        // 0 represents reserved
        return !check_fifth_bit(&byte);
        
    }

    pub fn is_safe_to_copy(&self) -> bool {
        let byte = self.safe_to_copy;
        let fifth_bit = (byte & (1 << 5)) >> 5;

        // 1 represents is safe
        return check_fifth_bit(&byte);
        
    }

}


fn check_fifth_bit(byte :&u8) -> bool {
    let fifth_bit = (byte & (1 << 5)) >> 5;

    if fifth_bit == 1 {return true }
    else {return false;}
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
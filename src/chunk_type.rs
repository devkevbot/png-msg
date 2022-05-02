#![allow(dead_code)]

use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct ChunkType {
    data: [u8; 4],
}

impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        self.data
    }

    fn is_valid(&self) -> bool {
        for b in self.data.iter() {
            if !b.is_ascii_alphabetic() {
                return false;
            }
        }

        self.is_reserved_bit_valid()
    }

    fn is_critical(&self) -> bool {
        let ancillary_byte = self.data[0];
        ancillary_byte.is_ascii_uppercase()
    }

    fn is_public(&self) -> bool {
        let private_byte = self.data[1];
        private_byte.is_ascii_uppercase()
    }

    fn is_reserved_bit_valid(&self) -> bool {
        let reserved_byte = self.data[2];
        reserved_byte.is_ascii_uppercase()
    }

    fn is_safe_to_copy(&self) -> bool {
        let safe_to_copy_byte = self.data[3];
        safe_to_copy_byte.is_ascii_lowercase()
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = String;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        for b in value.iter() {
            if !b.is_ascii_alphabetic() {
                return Err(String::from("expected A-Za-z"));
            }
        }
        Ok(ChunkType { data: value })
    }
}

impl FromStr for ChunkType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 && s.len() != 0 {
            return Err(String::from("must be length 0 or 4"));
        }

        let bytes_slice = s.as_bytes();
        for b in bytes_slice.iter() {
            if !b.is_ascii_alphabetic() {
                return Err(String::from("expected A-Za-z"));
            }
        }

        let bytes_array = bytes_slice.try_into();
        return match bytes_array {
            Ok(data) => Ok(ChunkType { data }),
            Err(e) => Err(e.to_string()),
        };
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let vector = Vec::from(self.data);
        let string = String::from_utf8(vector).expect("failed to convert Vec<u8> into String");
        write!(f, "{}", string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert!(!chunk.is_valid());

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

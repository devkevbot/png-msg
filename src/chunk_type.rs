#![allow(dead_code)]

use crate::{Error, Result};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

const CHUNK_TYPE_MAX_SIZE: usize = 4;

#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType {
    data: [u8; CHUNK_TYPE_MAX_SIZE],
}

impl ChunkType {
    const ANCILLARY_BYTE_INDEX: usize = 0;
    const PRIVATE_BYTE_INDEX: usize = Self::ANCILLARY_BYTE_INDEX + 1;
    const RESERVED_BYTE_INDEX: usize = Self::PRIVATE_BYTE_INDEX + 1;
    const SAFE_TO_COPY_BYTE_INDEX: usize = Self::RESERVED_BYTE_INDEX + 1;

    pub fn bytes(&self) -> [u8; CHUNK_TYPE_MAX_SIZE] {
        self.data
    }

    pub fn is_valid(&self) -> bool {
        for b in self.data.iter() {
            if !b.is_ascii_alphabetic() {
                return false;
            }
        }

        self.is_reserved_bit_valid()
    }

    pub fn is_critical(&self) -> bool {
        let ancillary_byte = self.data[Self::ANCILLARY_BYTE_INDEX];
        ancillary_byte.is_ascii_uppercase()
    }

    pub fn is_public(&self) -> bool {
        let private_byte = self.data[Self::PRIVATE_BYTE_INDEX];
        private_byte.is_ascii_uppercase()
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        let reserved_byte = self.data[Self::RESERVED_BYTE_INDEX];
        reserved_byte.is_ascii_uppercase()
    }

    pub fn is_safe_to_copy(&self) -> bool {
        let safe_to_copy_byte = self.data[Self::SAFE_TO_COPY_BYTE_INDEX];
        safe_to_copy_byte.is_ascii_lowercase()
    }
}

impl TryFrom<[u8; CHUNK_TYPE_MAX_SIZE]> for ChunkType {
    type Error = Error;

    fn try_from(value: [u8; CHUNK_TYPE_MAX_SIZE]) -> Result<Self> {
        for b in value.iter() {
            if !b.is_ascii_alphabetic() {
                return Err(Box::from(ChunkTypeError::InvalidCharacter(*b)));
            }
        }
        Ok(ChunkType { data: value })
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() != CHUNK_TYPE_MAX_SIZE && s.len() != 0 {
            return Err(Box::from(ChunkTypeError::InvalidLength(s.len())));
        }

        let bytes_slice = s.as_bytes();
        for b in bytes_slice.iter() {
            if !b.is_ascii_alphabetic() {
                return Err(Box::from(ChunkTypeError::InvalidCharacter(*b)));
            }
        }

        let bytes_array = bytes_slice.try_into()?;
        Ok(ChunkType { data: bytes_array })
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let vector = Vec::from(self.data);
        write!(f, "{}", String::from_utf8(vector).unwrap())
    }
}

#[derive(Debug)]
pub enum ChunkTypeError {
    InvalidCharacter(u8),
    InvalidLength(usize),
}

impl std::error::Error for ChunkTypeError {}

impl Display for ChunkTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ChunkTypeError::InvalidLength(length) => {
                write!(
                    f,
                    "expected 0 <= length <= {}, got {}",
                    length, CHUNK_TYPE_MAX_SIZE
                )
            }
            ChunkTypeError::InvalidCharacter(c) => write!(f, "unrecognized character {}", c),
        }
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

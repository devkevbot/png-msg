#![allow(dead_code)]
#![allow(unused_variables)]

use crate::chunk_type::ChunkType;
use crate::{Error, Result};
use std::fmt::{Display, Formatter};

pub struct Chunk {
    chunk_type: ChunkType,
    data: Vec<u8>,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        Self { chunk_type, data }
    }

    pub const CHUNK_LENGTH_FIELD_SIZE: usize = 4;
    pub const CHUNK_TYPE_FIELD_SIZE: usize = 4;
    pub const CHUNK_CRC_FIELD_SIZE: usize = 4;

    pub const CHUNK_META_SIZE: usize =
        Self::CHUNK_LENGTH_FIELD_SIZE + Self::CHUNK_TYPE_FIELD_SIZE + Self::CHUNK_CRC_FIELD_SIZE;

    pub fn length(&self) -> usize {
        self.data.len()
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn crc(&self) -> u32 {
        let bytes: Vec<u8> = self
            .chunk_type
            .bytes()
            .iter()
            .chain(self.data.iter())
            .copied()
            .collect();
        crc::crc32::checksum_ieee(&bytes)
    }

    pub fn data_as_string(&self) -> Result<String> {
        let s = std::str::from_utf8(&self.data)?;
        Ok(s.to_string())
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let data_length = self.data.len() as u32;
        data_length
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data.iter())
            .chain(self.crc().to_be_bytes().iter())
            .copied()
            .collect()
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        let (length, value) = value.split_at(Self::CHUNK_LENGTH_FIELD_SIZE);
        let length = u32::from_be_bytes(length.try_into()?) as usize;

        let (chunk_type, value) = value.split_at(Self::CHUNK_TYPE_FIELD_SIZE);
        let chunk_type: [u8; Self::CHUNK_LENGTH_FIELD_SIZE] = chunk_type.try_into()?;
        let chunk_type = ChunkType::try_from(chunk_type)?;

        if !chunk_type.is_valid() {
            return Err(Box::from(ChunkParseError::BadChunkType));
        }

        let (data, value) = value.split_at(length);

        let (crc, value) = value.split_at(Self::CHUNK_CRC_FIELD_SIZE);

        let supplied_chunk = Self {
            chunk_type,
            data: data.into(),
        };

        let actual_crc = supplied_chunk.crc();
        let expected_crc = u32::from_be_bytes(crc.try_into()?);

        if actual_crc != expected_crc {
            return Err(Box::from(ChunkParseError::BadCrc(expected_crc, actual_crc)));
        }

        Ok(supplied_chunk)
    }
}

#[derive(Debug)]
pub enum ChunkParseError {
    BadCrc(u32, u32),
    BadChunkType,
}

impl std::error::Error for ChunkParseError {}

impl Display for ChunkParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ChunkParseError::BadCrc(expected, actual) => {
                write!(f, "expected CRC of {} but got CRC of {}", expected, actual)
            }
            ChunkParseError::BadChunkType => write!(f, "unrecognized chunk type"),
        }
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
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
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
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

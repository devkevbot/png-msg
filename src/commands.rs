use crate::png::{Chunk, ChunkType, Png};
use crate::{Error, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub fn encode(
    input_path: PathBuf,
    chunk_type: String,
    message: String,
    output_path: Option<PathBuf>,
) -> Result<()> {
    let chunk_type = ChunkType::from_str(&chunk_type)?;
    let data = message.as_bytes().to_vec();
    let chunk = Chunk::new(chunk_type, data);

    let mut png = from_file(&input_path)?;

    println!("{}", png);

    png.append_chunk(chunk);

    println!("{}", png);

    let bytes = png.as_bytes();

    match output_path {
        Some(output_path) => Ok(to_file(&output_path, bytes)?),
        None => Ok(()),
    }
}

pub fn decode(input_path: PathBuf, chunk_type: String) -> Result<()> {
    let png = from_file(&input_path)?;
    if let Some(chonk) = png.chunk_by_type(&chunk_type) {
        let data = Chunk::data_as_string(chonk)?;
        println!("Secret message was: {}", data);
    }

    Ok(())
}

pub fn remove(input_path: PathBuf, chunk_type: String) -> Result<()> {
    let mut png = from_file(&input_path)?;
    let removed_chunk = png.remove_chunk(&chunk_type)?;
    println!("Removed chunk {}", removed_chunk.data_as_string()?);

    let bytes = png.as_bytes();

    to_file(&input_path, bytes)?;

    Ok(())
}

fn from_file<P: AsRef<Path>>(path: &P) -> Result<Png> {
    let file_contents = fs::read(path)?;
    let png = Png::try_from(file_contents.as_slice())?;
    Ok(png)
}

fn to_file<P: AsRef<Path>>(path: &P, contents: Vec<u8>) -> Result<()> {
    fs::write(path, contents)?;
    Ok(())
}

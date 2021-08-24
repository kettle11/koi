use kserde::*;
use std::{borrow::Cow, io::Read};

use crate::GlTf;

#[derive(Debug, Clone)]
/// A glTF with all of its data included within a buffer.
pub struct GLB<'a> {
    /// The GLTF data decoded from the Json embedded in the GLB.
    pub gltf: GlTf,
    /// The version of the GLB file. This is different from the glTF version.
    pub glb_version: u32,
    /// The binary data buffer that is referenced from the glTF.
    pub binary_data: Option<Cow<'a, [u8]>>,
}

#[derive(Debug)]
pub enum GLBError {
    Io(::std::io::Error),
    /// The file's magic number is incorrect. This probably isn't a GLB.
    IncorrectMagicNumber,
    /// The file's formatting is incorrect.
    IncorrectFormatting,
    /// The GLB's inner JSON is incorrectly formatted or could not be parsed.
    InvalidJSON,
}

impl<'a> GLB<'a> {
    pub fn from_bytes(data: &'a [u8]) -> Result<Self, GLBError> {
        // This function extra copies.
        // That could be improved in the future.
        let reader = std::io::BufReader::new(data);
        Self::from_reader(reader)
    }

    pub fn from_reader<R: Read>(mut reader: R) -> Result<Self, GLBError> {
        // Header
        let magic = reader.get_u32()?;
        if magic != 0x46546C67 {
            Err(GLBError::IncorrectMagicNumber)?
        }

        let glb_version = reader.get_u32()?;
        let _file_length = reader.get_u32()?;

        // JSON Chunk
        let json_chunk_length = reader.get_u32()?;
        let json_chunk_type = reader.get_u32()?;
        if json_chunk_type != 0x4E4F534A {
            // The chunk type does not match the expected chunk type
            Err(GLBError::IncorrectFormatting)?
        }

        let mut json_string_bytes = vec![0; json_chunk_length as usize];
        reader
            .read_exact(&mut json_string_bytes)
            .map_err(GLBError::Io)?;

        let json_string =
            String::from_utf8(json_string_bytes).map_err(|_| GLBError::IncorrectFormatting)?;
        let gltf = GlTf::from_json(&json_string).ok_or(GLBError::InvalidJSON)?;

        let mut binary_data = None;
        if let Ok(binary_chunk_length) = reader.get_u32() {
            let binary_chunk_type = reader.get_u32()?;
            if binary_chunk_type == 0x004E4942 {
                let mut binary_data_buffer = vec![0; binary_chunk_length as usize];
                reader
                    .read_exact(&mut binary_data_buffer)
                    .map_err(GLBError::Io)?;
                binary_data = Some(binary_data_buffer.into());
            }
        }

        Ok(GLB {
            gltf,
            glb_version,
            binary_data,
        })
    }
}

trait ReaderExtensions: Read {
    fn get_u32(&mut self) -> Result<u32, GLBError> {
        let mut bytes = [0; 4];
        self.read_exact(&mut bytes).map_err(GLBError::Io)?;
        Ok(u32::from_le_bytes(bytes))
    }
}

impl<R: Read> ReaderExtensions for R {}

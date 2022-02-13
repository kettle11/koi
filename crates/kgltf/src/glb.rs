use kserde::*;
use std::{borrow::Cow, io::Read, io::Write};

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
const GLB_MAGIC_NUMBER: u32 = 0x46546C67;
const JSON_CHUNK_TYPE: u32 = 0x4E4F534A;
const BIN_CHUNK_TYPE: u32 = 0x004E4942;

impl<'a> GLB<'a> {
    pub fn from_bytes(data: &'a [u8]) -> Result<Self, GLBError> {
        let reader = std::io::BufReader::new(data);
        Self::from_reader(reader)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut writer = Vec::new();
        self.to_writer(&mut writer)?;
        Ok(writer)
    }

    pub fn from_reader<R: Read>(mut reader: R) -> Result<Self, GLBError> {
        // Header
        let magic = reader.get_u32()?;
        if magic != GLB_MAGIC_NUMBER {
            return Err(GLBError::IncorrectMagicNumber);
        }

        let glb_version = reader.get_u32()?;
        let _file_length = reader.get_u32()?;

        // JSON Chunk
        let json_chunk_length = reader.get_u32()?;
        let json_chunk_type = reader.get_u32()?;

        if json_chunk_type != JSON_CHUNK_TYPE {
            // The chunk type does not match the expected chunk type
            return Err(GLBError::IncorrectFormatting);
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
            if binary_chunk_type == BIN_CHUNK_TYPE {
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

    pub fn to_writer<W: Write>(&self, mut writer: W) -> Result<(), std::io::Error> {
        let json = self.gltf.to_json();
        let json_length: u32 = to_power_of_4(json.len()) as u32;
        let json_padding = json_length - json.len() as u32;

        let data_padding = self
            .binary_data
            .as_ref()
            .map_or(0, |d| to_power_of_4(d.len()) - d.len()) as u32;

        let file_length: u32 = 4 * 5
            + json_length
            + self
                .binary_data
                .as_ref()
                .map_or(0, |b| 4 * 2 + b.len() as u32)
            + data_padding;

        // Write magic number
        writer.write(&GLB_MAGIC_NUMBER.to_le_bytes())?; // 1 counting u32s written
        writer.write(&self.glb_version.to_le_bytes())?; // 2
        writer.write(&file_length.to_le_bytes())?; // 3

        writer.write(&(json_length).to_le_bytes())?; // 4
        writer.write(&JSON_CHUNK_TYPE.to_le_bytes())?; // 5
        writer.write(json.as_bytes())?;

        for _ in 0..json_padding {
            writer.write(&[0x20])?;
        }

        if let Some(data) = &self.binary_data {
            writer.write(&(data.len() as u32 + data_padding).to_le_bytes())?;
            writer.write(&BIN_CHUNK_TYPE.to_le_bytes())?;
            writer.write(&data)?;

            for _ in 0..data_padding {
                writer.write(&[0])?;
            }
        }

        Ok(())
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

fn to_power_of_4(n: usize) -> usize {
    let remainder = n % 4;
    if remainder == 0 {
        n
    } else {
        n + 4 - remainder
    }
}

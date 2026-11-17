use std::io;
use std::io::prelude::*;
use std::str::FromStr;
use std::{fs::File, path::Path, path::PathBuf};

use rand_core::Rng;
use strum_macros::{AsRefStr, Display, EnumString};

#[derive(Clone, Copy, EnumString, AsRefStr, Display, PartialEq, Debug)]
#[strum(serialize_all = "lowercase")]
pub enum Extension {
    Zip,
    Pdf,
    Doc,
    Txt,
}

impl Extension {
    pub fn header(&self) -> Vec<u8> {
        match self {
            Extension::Zip => vec![
                0x50, 0x4b, 0x03, 0x04, 0x0a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x26, 0x79, 0x5d, 0x40,
                0xde, 0xbd, 0xac, 0x82, 0x00, 0x04, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x0a, 0x00,
                0x1c, 0x00,
            ],
            Extension::Pdf => vec![
                0x25, 0x50, 0x44, 0x46, 0x2d, 0x31, 0x2e, 0x34, 0x0a, 0x25, 0xe1, 0xe9, 0xeb,
            ],
            Extension::Doc => vec![0xd0, 0xcf, 0x11, 0xe0, 0xa1, 0xb1, 0x1a, 0xe1],
            _ => vec![],
        }
    }
}

#[derive(Debug)]
pub struct FakeFile {
    filename: PathBuf,
    size: u64,
    file_type: Extension,
}

impl FakeFile {
    /// Creates a new FakeFile with the specified parameters
    pub fn new(filename: PathBuf, size: u64, file_type: Extension) -> Self {
        Self {
            filename,
            size,
            file_type,
        }
    }

    /// Infers the file type from the filename extension, defaulting to Txt
    pub fn infer_type_from_filename(filename: &Path) -> Extension {
        filename
            .extension()
            .and_then(|ext| ext.to_str())
            .and_then(|ext| Extension::from_str(ext).ok())
            .unwrap_or(Extension::Txt)
    }

    /// Creates a FakeFile with file type inferred from filename
    pub fn from_filename_and_size(filename: PathBuf, size: u64) -> Self {
        let file_type = Self::infer_type_from_filename(&filename);
        Self::new(filename, size, file_type)
    }

    /// Gets the output filename with the correct extension
    pub fn output_filename(&self) -> PathBuf {
        self.filename.with_extension(self.file_type.as_ref())
    }

    /// Creates the file buffer with header and random data
    pub fn create_buffer<R: Rng>(&self, rng: &mut R) -> Vec<u8> {
        let header = self.file_type.header();
        let remaining_size = self.size.saturating_sub(header.len() as u64) as usize;

        let mut buffer = Vec::with_capacity(self.size as usize);
        buffer.extend_from_slice(&header);

        let mut rest = vec![0u8; remaining_size];
        rng.fill_bytes(&mut rest);
        buffer.extend_from_slice(&rest);

        buffer
    }

    /// Writes the fake file to disk
    pub fn write_to_disk<R: Rng>(&self, rng: &mut R) -> io::Result<()> {
        let buffer = self.create_buffer(rng);
        let output_path = self.output_filename();

        let mut file = File::create(output_path)?;
        file.write_all(&buffer)?;

        Ok(())
    }
}

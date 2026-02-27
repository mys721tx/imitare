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

#[cfg(test)]
mod tests {
    use super::*;
    use chacha20::ChaCha12Rng;
    use rand_core::SeedableRng;
    use std::io::Read;
    use tempfile::TempDir;

    #[test]
    fn test_extension_from_str() {
        assert_eq!(Extension::from_str("zip").unwrap(), Extension::Zip);
        assert_eq!(Extension::from_str("pdf").unwrap(), Extension::Pdf);
        assert_eq!(Extension::from_str("doc").unwrap(), Extension::Doc);
        assert_eq!(Extension::from_str("txt").unwrap(), Extension::Txt);
        assert!(Extension::from_str("unknown").is_err());
    }

    #[test]
    fn test_extension_headers() {
        // ZIP header test - should start with PK signature
        let zip_header = Extension::Zip.header();
        assert_eq!(zip_header[0..4], [0x50, 0x4b, 0x03, 0x04]);

        // PDF header test - should start with %PDF-1.4
        let pdf_header = Extension::Pdf.header();
        assert_eq!(
            pdf_header[0..8],
            [0x25, 0x50, 0x44, 0x46, 0x2d, 0x31, 0x2e, 0x34]
        );

        // DOC header test - should match compound document signature
        let doc_header = Extension::Doc.header();
        assert_eq!(
            doc_header,
            vec![0xd0, 0xcf, 0x11, 0xe0, 0xa1, 0xb1, 0x1a, 0xe1]
        );

        // TXT header test - should be empty
        let txt_header = Extension::Txt.header();
        assert!(txt_header.is_empty());
    }

    #[test]
    fn test_fake_file_new() {
        let filename = PathBuf::from("test.pdf");
        let size = 1024;
        let file_type = Extension::Pdf;

        let fake_file = FakeFile::new(filename.clone(), size, file_type);

        // Test that the output filename is correct
        assert_eq!(fake_file.output_filename(), PathBuf::from("test.pdf"));

        // Test that the buffer has the correct size and header
        let mut rng = ChaCha12Rng::seed_from_u64(42);
        let buffer = fake_file.create_buffer(&mut rng);
        assert_eq!(buffer.len(), size as usize);

        let header = Extension::Pdf.header();
        assert_eq!(&buffer[0..header.len()], &header[..]);
    }

    #[test]
    fn test_infer_type_from_filename() {
        assert_eq!(
            FakeFile::infer_type_from_filename(&PathBuf::from("test.pdf")),
            Extension::Pdf
        );
        assert_eq!(
            FakeFile::infer_type_from_filename(&PathBuf::from("test.zip")),
            Extension::Zip
        );
        assert_eq!(
            FakeFile::infer_type_from_filename(&PathBuf::from("test.doc")),
            Extension::Doc
        );
        assert_eq!(
            FakeFile::infer_type_from_filename(&PathBuf::from("test.txt")),
            Extension::Txt
        );
        assert_eq!(
            FakeFile::infer_type_from_filename(&PathBuf::from("test")),
            Extension::Txt
        );
        assert_eq!(
            FakeFile::infer_type_from_filename(&PathBuf::from("test.unknown")),
            Extension::Txt
        );
    }

    #[test]
    fn test_from_filename_and_size() {
        let fake_file = FakeFile::from_filename_and_size(PathBuf::from("test.pdf"), 1024);

        // Test that it correctly inferred PDF type by checking the buffer has PDF header
        let mut rng = ChaCha12Rng::seed_from_u64(42);
        let buffer = fake_file.create_buffer(&mut rng);
        assert_eq!(buffer.len(), 1024);

        let header = Extension::Pdf.header();
        assert_eq!(&buffer[0..header.len()], &header[..]);
    }

    #[test]
    fn test_output_filename() {
        let fake_file = FakeFile::new(PathBuf::from("test"), 1024, Extension::Pdf);
        assert_eq!(fake_file.output_filename(), PathBuf::from("test.pdf"));

        let fake_file = FakeFile::new(PathBuf::from("test.old"), 1024, Extension::Zip);
        assert_eq!(fake_file.output_filename(), PathBuf::from("test.zip"));
    }

    #[test]
    fn test_create_buffer_with_header() {
        let fake_file = FakeFile::new(PathBuf::from("test.pdf"), 100, Extension::Pdf);
        let mut rng = ChaCha12Rng::seed_from_u64(42);

        let buffer = fake_file.create_buffer(&mut rng);
        let header = Extension::Pdf.header();

        // Buffer should be exactly the requested size
        assert_eq!(buffer.len(), 100);

        // Buffer should start with the PDF header
        assert_eq!(&buffer[0..header.len()], &header[..]);

        // Remaining bytes should be random (non-zero in this case due to our seed)
        let remaining = &buffer[header.len()..];
        assert!(
            !remaining.iter().all(|&b| b == 0),
            "Remaining bytes should be random"
        );
    }

    #[test]
    fn test_create_buffer_small_size() {
        // Test when requested size is smaller than header
        let fake_file = FakeFile::new(PathBuf::from("test.pdf"), 5, Extension::Pdf);
        let mut rng = ChaCha12Rng::seed_from_u64(42);

        let buffer = fake_file.create_buffer(&mut rng);
        let header = Extension::Pdf.header();

        // Buffer should still contain the full header
        assert_eq!(&buffer[0..header.len()], &header[..]);

        // The actual buffer size should be at least as large as the header
        assert!(buffer.len() >= header.len());
    }

    #[test]
    fn test_create_buffer_txt_file() {
        let fake_file = FakeFile::new(PathBuf::from("test.txt"), 100, Extension::Txt);
        let mut rng = ChaCha12Rng::seed_from_u64(42);

        let buffer = fake_file.create_buffer(&mut rng);

        // Buffer should be exactly the requested size
        assert_eq!(buffer.len(), 100);

        // All bytes should be random (no header for txt files)
        assert!(
            !buffer.iter().all(|&b| b == 0),
            "All bytes should be random for txt files"
        );
    }

    #[test]
    fn test_write_to_disk() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.pdf");

        let fake_file = FakeFile::new(file_path.clone(), 1000, Extension::Pdf);
        let mut rng = ChaCha12Rng::seed_from_u64(42);

        // Write the file
        fake_file.write_to_disk(&mut rng).unwrap();

        // Verify the file was created
        let output_path = fake_file.output_filename();
        assert!(output_path.exists());

        // Verify file contents
        let mut file_contents = Vec::new();
        let mut file = File::open(&output_path).unwrap();
        file.read_to_end(&mut file_contents).unwrap();

        // Check file size
        assert_eq!(file_contents.len(), 1000);

        // Check PDF header
        let header = Extension::Pdf.header();
        assert_eq!(&file_contents[0..header.len()], &header[..]);
    }

    #[test]
    fn test_write_to_disk_with_relative_path() {
        let temp_dir = TempDir::new().unwrap();

        // Change to temp directory for this test
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(&temp_dir).unwrap();

        let fake_file = FakeFile::new(PathBuf::from("relative_test.zip"), 500, Extension::Zip);
        let mut rng = ChaCha12Rng::seed_from_u64(42);

        // Write the file
        fake_file.write_to_disk(&mut rng).unwrap();

        // Verify the file was created in current directory
        assert!(PathBuf::from("relative_test.zip").exists());

        // Restore original directory
        std::env::set_current_dir(original_dir).unwrap();
    }

    #[test]
    fn test_different_file_types_have_different_headers() {
        let mut rng = ChaCha12Rng::seed_from_u64(42);

        let pdf_file = FakeFile::new(PathBuf::from("test.pdf"), 100, Extension::Pdf);
        let zip_file = FakeFile::new(PathBuf::from("test.zip"), 100, Extension::Zip);
        let doc_file = FakeFile::new(PathBuf::from("test.doc"), 100, Extension::Doc);

        let pdf_buffer = pdf_file.create_buffer(&mut rng);
        let zip_buffer = zip_file.create_buffer(&mut rng);
        let doc_buffer = doc_file.create_buffer(&mut rng);

        // All buffers should have different starting bytes
        assert_ne!(&pdf_buffer[0..4], &zip_buffer[0..4]);
        assert_ne!(&pdf_buffer[0..4], &doc_buffer[0..4]);
        assert_ne!(&zip_buffer[0..4], &doc_buffer[0..4]);

        // PDF should start with %PDF
        assert_eq!(&pdf_buffer[0..4], &[0x25, 0x50, 0x44, 0x46]);

        // ZIP should start with PK
        assert_eq!(&zip_buffer[0..4], &[0x50, 0x4b, 0x03, 0x04]);

        // DOC should start with compound document signature
        assert_eq!(&doc_buffer[0..4], &[0xd0, 0xcf, 0x11, 0xe0]);
    }

    #[test]
    fn test_zero_size_file() {
        let fake_file = FakeFile::new(PathBuf::from("test.pdf"), 0, Extension::Pdf);
        let mut rng = ChaCha12Rng::seed_from_u64(42);

        let buffer = fake_file.create_buffer(&mut rng);
        let header = Extension::Pdf.header();

        // Even with zero size, we should get at least the header
        assert!(buffer.len() >= header.len());
        assert_eq!(&buffer[0..header.len()], &header[..]);
    }
}

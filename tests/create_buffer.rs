use std::path::PathBuf;

use chacha20::ChaCha12Rng;
use imitare::{Extension, FakeFile};
use rand_core::SeedableRng;

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

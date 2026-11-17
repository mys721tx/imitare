use std::path::PathBuf;

use chacha20::ChaCha12Rng;
use imitare::{Extension, FakeFile};
use rand_core::SeedableRng;

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

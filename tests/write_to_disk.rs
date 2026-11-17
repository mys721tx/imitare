use std::fs::File;
use std::io::Read;

use chacha20::ChaCha12Rng;
use imitare::{Extension, FakeFile};
use rand_core::SeedableRng;
use tempfile::TempDir;

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

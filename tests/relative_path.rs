//! Kept in its own test binary because it changes the process-wide current
//! directory, which would race with other tests running in parallel.

use std::path::PathBuf;

use chacha20::ChaCha12Rng;
use imitare::{Extension, FakeFile};
use rand_core::SeedableRng;
use tempfile::TempDir;

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

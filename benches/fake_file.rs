use std::hint::black_box;
use std::path::PathBuf;

use chacha20::ChaCha12Rng;
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use imitare::{Extension, FakeFile};
use rand_core::SeedableRng;
use tempfile::TempDir;

const SIZES: [u64; 4] = [4 * 1024, 64 * 1024, 1024 * 1024, 16 * 1024 * 1024];
const TYPES: [Extension; 4] = [
    Extension::Txt,
    Extension::Pdf,
    Extension::Zip,
    Extension::Doc,
];

fn bench_header(c: &mut Criterion) {
    let mut group = c.benchmark_group("header");
    for ext in TYPES {
        group.bench_with_input(BenchmarkId::from_parameter(ext), &ext, |b, ext| {
            b.iter(|| black_box(ext).header())
        });
    }
    group.finish();
}

fn bench_infer_type(c: &mut Criterion) {
    let mut group = c.benchmark_group("infer_type_from_filename");
    for name in ["output.pdf", "output.zip", "output", "output.unknown"] {
        let path = PathBuf::from(name);
        group.bench_with_input(BenchmarkId::from_parameter(name), &path, |b, path| {
            b.iter(|| FakeFile::infer_type_from_filename(black_box(path)))
        });
    }
    group.finish();
}

fn bench_create_buffer(c: &mut Criterion) {
    let mut group = c.benchmark_group("create_buffer");
    for size in SIZES {
        group.throughput(Throughput::Bytes(size));
        let fake_file = FakeFile::new(PathBuf::from("bench.pdf"), size, Extension::Pdf);
        let mut rng = ChaCha12Rng::seed_from_u64(42);
        group.bench_with_input(BenchmarkId::from_parameter(size), &fake_file, |b, f| {
            b.iter(|| f.create_buffer(&mut rng))
        });
    }
    group.finish();
}

fn bench_write_to_disk(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    let mut group = c.benchmark_group("write_to_disk");
    group.sample_size(20);
    for size in SIZES {
        group.throughput(Throughput::Bytes(size));
        let fake_file = FakeFile::new(temp_dir.path().join("bench.pdf"), size, Extension::Pdf);
        let mut rng = ChaCha12Rng::seed_from_u64(42);
        group.bench_with_input(BenchmarkId::from_parameter(size), &fake_file, |b, f| {
            b.iter(|| f.write_to_disk(&mut rng).unwrap())
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_header,
    bench_infer_type,
    bench_create_buffer,
    bench_write_to_disk
);
criterion_main!(benches);

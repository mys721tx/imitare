use std::io::prelude::*;
use std::fs::File;
use rand::prelude::*;

fn main() {
    let mut buffer = File::create("foo.txt").unwrap();
    let mut rng = StdRng::from_entropy();

    let mut arr = [0u8; 256];

    for _ in 0..10 {
        rng.fill_bytes(&mut arr);

        buffer.write_all(&arr).unwrap();
    }
}

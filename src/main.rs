use std::io::prelude::*;
use std::fs::File;
use rand::prelude::*;

fn main() {
    let mut buffer = File::create("foo.txt").unwrap();
    let mut rng = StdRng::from_entropy();

    let mut arr = [0u8; 256];

    let size = 854_235;

    for _ in 0..(size / arr.len()) {
        rng.fill_bytes(&mut arr);

        buffer.write_all(&arr).unwrap();
    }

    // Handling the remainder
    rng.fill_bytes(&mut arr);

    let mut r = arr.to_vec();
    r.truncate(size % arr.len());

    buffer.write_all(r.as_slice()).unwrap();
}

extern crate rand;

use std::io::prelude::*;
use std::fs::File;
use rand::prelude::*;

fn main() {
    let mut buffer = File::create("foo.txt").unwrap();
    let mut rng = StdRng::from_entropy();

    let mut arr = [0u8; 128];

    rng.fill_bytes(&mut arr);

    buffer.write_all(&arr).unwrap();
}

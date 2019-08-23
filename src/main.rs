use std::io;
use std::io::prelude::*;
use std::fs::File;
use rand::prelude::*;

fn main() {
    let mut buffer = File::create("foo.txt").unwrap();
    let mut rng = StdRng::from_entropy();

    let r = &mut rng as &mut dyn RngCore;

    let size = 854_235;

    io::copy(&mut r.take(size), &mut buffer).unwrap();
}

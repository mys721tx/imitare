use clap::{App, Arg};
use rand::prelude::*;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() {
    let mut rng = StdRng::from_entropy();

    let matches = App::new("imitare")
        .version("0.1")
        .author("Yishen Miao <mys721tx@gmail.com>")
        .about("A fake file generator")
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("out")
                .value_name("FILE")
                .help("Sets the name of the output file. Defaults to \"output.txt\".")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("size")
                .short("s")
                .long("size")
                .value_name("SIZE")
                .help("Sets the size of the output in byte. Defaults to 4096.")
                .takes_value(true),
        )
        .get_matches();

    let file = matches.value_of("output").unwrap_or("output.txt");
    let size = matches
        .value_of("size")
        .and_then(|x| x.parse::<u64>().ok())
        .unwrap_or(4096);
    let mut buffer = File::create(file).unwrap();

    let r = &mut rng as &mut dyn RngCore;

    io::copy(&mut r.take(size), &mut buffer).unwrap();
}

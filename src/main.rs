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
                .help("Sets the size of the output in byte. Defaults to 4096. If a type is set, \
                the type header will be written regardless of size. Any remaining bytes will be \
                written afterward.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("type")
                .short("t")
                .long("type")
                .value_name("TYPE")
                .help("Sets the header type of the output. Not enabled by default.")
                .takes_value(true),
        )
        .get_matches();

    let file = matches.value_of("output").unwrap_or("output.txt");
    let size = matches
        .value_of("size")
        .and_then(|x| x.parse::<u64>().ok())
        .unwrap_or(4096);

    let header = match matches.value_of("type") {
        Some("zip") => vec![
            0x50, 0x4b, 0x3, 0x4, 0xa, 0x0, 0x0, 0x0, 0x0, 0x0, 0x26, 0x79, 0x5d, 0x40, 0xde, 0xbd,
            0xac, 0x82, 0x0, 0x4, 0x0, 0x0, 0x0, 0x4, 0x0, 0x0, 0xa, 0x0, 0x1c, 0x0,
        ],
        Some("pdf") => vec![
            0x25, 0x50, 0x44, 0x46, 0x2d, 0x31, 0x2e, 0x34, 0xa, 0x25, 0xe1, 0xe9, 0xeb,
        ],
        Some("doc") => vec![0xd0, 0xcf, 0x11, 0xe0, 0xa1, 0xb1, 0x1a, 0xe1],
        _ => vec![],
    };

    let mut buffer = File::create(file).unwrap();

    let r = &mut rng as &mut dyn RngCore;

    buffer.write_all(&header).unwrap();

    io::copy(
        &mut r.take(size.saturating_sub(header.len() as u64)),
        &mut buffer,
    )
    .unwrap();
}

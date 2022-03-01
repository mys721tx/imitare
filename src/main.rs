use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

use clap::{Arg, Command};
use rand::prelude::*;

fn main() {
    let mut rng = StdRng::from_entropy();

    let matches = Command::new("imitare")
        .version("0.1")
        .author("Yishen Miao <mys721tx@gmail.com>")
        .about("A fake file generator")
        .arg(
            Arg::new("output")
                .short('o')
                .long("out")
                .value_name("FILE")
                .help("Sets the name of the output file. Defaults to \"output.txt\".")
                .takes_value(true),
        )
        .arg(
            Arg::new("size")
                .short('s')
                .long("size")
                .value_name("SIZE")
                .help(
                    "Sets the size of the output in byte. Defaults to 4096. If a type is set, \
                the type header will be written regardless of size. Any remaining bytes will be \
                written afterward.",
                )
                .takes_value(true),
        )
        .arg(
            Arg::new("type")
                .short('t')
                .long("type")
                .value_name("TYPE")
                .help(
                    "Sets the type of the output file. If not set, the file type is inferred \
                from the output extension. Defaults to txt.",
                )
                .takes_value(true),
        )
        .get_matches();

    let filename = Path::new(matches.value_of("output").unwrap_or("output"));
    let size = matches
        .value_of("size")
        .and_then(|x| x.parse::<u64>().ok())
        .unwrap_or(4096);
    let filetype = matches
        .value_of("type")
        .or_else(|| filename.extension().and_then(|x| x.to_str()))
        .unwrap_or("txt");

    let header = match filetype {
        "zip" => vec![
            0x50, 0x4b, 0x03, 0x04, 0x0a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x26, 0x79, 0x5d, 0x40,
            0xde, 0xbd, 0xac, 0x82, 0x00, 0x04, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x0a, 0x00,
            0x1c, 0x00,
        ],
        "pdf" => vec![
            0x25, 0x50, 0x44, 0x46, 0x2d, 0x31, 0x2e, 0x34, 0x0a, 0x25, 0xe1, 0xe9, 0xeb,
        ],
        "doc" => vec![0xd0, 0xcf, 0x11, 0xe0, 0xa1, 0xb1, 0x1a, 0xe1],
        _ => vec![],
    };

    let mut buffer = File::create(filename.with_extension(filetype)).unwrap();

    let r = &mut rng as &mut dyn RngCore;

    buffer.write_all(&header).unwrap();

    io::copy(
        &mut r.take(size.saturating_sub(header.len() as u64)),
        &mut buffer,
    )
    .unwrap();
}

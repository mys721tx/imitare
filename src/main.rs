use std::io;
use std::io::prelude::*;
use std::str::FromStr;
use std::{fs::File, path::PathBuf};

use clap::{value_parser, Arg, Command};
use rand::prelude::*;
use strum_macros::{AsRefStr, Display, EnumString};

#[derive(Clone, Copy, EnumString, AsRefStr, Display)]
#[strum(serialize_all = "lowercase")]
enum Extension {
    Zip,
    Pdf,
    Doc,
    Txt,
}

impl Extension {
    fn header(&self) -> Vec<u8> {
        match self {
            Extension::Zip => vec![
                0x50, 0x4b, 0x03, 0x04, 0x0a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x26, 0x79, 0x5d, 0x40,
                0xde, 0xbd, 0xac, 0x82, 0x00, 0x04, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x0a, 0x00,
                0x1c, 0x00,
            ],
            Extension::Pdf => vec![
                0x25, 0x50, 0x44, 0x46, 0x2d, 0x31, 0x2e, 0x34, 0x0a, 0x25, 0xe1, 0xe9, 0xeb,
            ],
            Extension::Doc => vec![0xd0, 0xcf, 0x11, 0xe0, 0xa1, 0xb1, 0x1a, 0xe1],
            _ => vec![],
        }
    }
}

fn main() {
    let mut rng = StdRng::from_os_rng();

    let matches = Command::new("imitare")
        .version("0.1")
        .author("Yishen Miao <mys721tx@gmail.com>")
        .about("A fake file generator")
        .args_override_self(true)
        .arg(
            Arg::new("output")
                .short('o')
                .long("out")
                .value_name("FILE")
                .value_parser(value_parser!(PathBuf))
                .default_value("output.txt")
                .action(clap::ArgAction::Set)
                .help("Sets the name of the output file. Defaults to \"output.txt\"."),
        )
        .arg(
            Arg::new("size")
                .short('s')
                .long("size")
                .value_name("SIZE")
                .value_parser(value_parser!(u64))
                .default_value("4096")
                .action(clap::ArgAction::Set)
                .help(
                    "Sets the size of the output in byte. Defaults to 4096. If a type is set, \
                the type header will be written regardless of size. Any remaining bytes will be \
                written afterward.",
                ),
        )
        .arg(
            Arg::new("type")
                .short('t')
                .long("type")
                .value_name("TYPE")
                .value_parser(value_parser!(Extension))
                .action(clap::ArgAction::Set)
                .help(
                    "Sets the type of the output file. If not set, the file type is inferred \
                from the output extension. Defaults to txt.",
                ),
        )
        .get_matches();

    let filename: &PathBuf = matches.get_one("output").unwrap();
    let size: &u64 = matches.get_one("size").unwrap();
    let ext = filename
        .extension()
        .and_then(|ext| ext.to_str())
        .and_then(|ext| Extension::from_str(ext).ok())
        .unwrap_or(Extension::Txt);
    let filetype: &Extension = matches.get_one::<Extension>("type").unwrap_or(&ext);

    let header = filetype.header();

    let mut buffer = File::create(filename.with_extension(filetype.as_ref())).unwrap();

    let mut rest = vec![0u8; size.saturating_sub(header.len() as u64) as usize];
    rng.fill_bytes(&mut rest);
    buffer.write_all(&header).unwrap();

    io::copy(&mut &rest[..], &mut buffer).unwrap();
}

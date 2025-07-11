use std::path::PathBuf;

use clap::{value_parser, Arg, Command};
use rand::prelude::*;

mod fake_file;
use fake_file::{Extension, FakeFile};

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

    // Create FakeFile instance
    let fake_file = if let Some(file_type) = matches.get_one::<Extension>("type") {
        // Use explicitly specified type
        FakeFile::new(filename.clone(), *size, *file_type)
    } else {
        // Infer type from filename
        FakeFile::from_filename_and_size(filename.clone(), *size)
    };

    // Generate and write the file
    if let Err(e) = fake_file.write_to_disk(&mut rng) {
        eprintln!("Error creating file: {}", e);
        std::process::exit(1);
    }
}

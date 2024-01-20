use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::ExitCode;
use std::{env, fs};

use clap::{command, Arg};

fn main() -> ExitCode {
    let matches = command!()
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .help("Sets file to rename")
                .value_name("FILE")
                .value_parser(clap::value_parser!(PathBuf))
                .num_args(1)
                .required(false),
        )
        .arg(
            Arg::new("year")
                .required(true)
                .help("Year the work was published in")
                .required(true)
                .index(1)
                .value_parser(clap::value_parser!(u64).range(0..)),
        )
        .arg(
            Arg::new("conf")
                .help("Conference the work was published in")
                .required(true)
                .index(2)
                .value_parser(clap::builder::NonEmptyStringValueParser::new()),
        )
        .arg(
            Arg::new("title")
                .help("Title the work was published with, surrounded by quotes")
                .required(true)
                .index(3)
                .value_parser(clap::builder::NonEmptyStringValueParser::new()),
        )
        .get_matches();

    let year: &u64 = matches.get_one::<u64>("year").unwrap();
    let conf: String = matches
        .get_one::<String>("conf")
        .unwrap()
        .trim()
        .to_ascii_lowercase();
    let title: String = matches
        .get_one::<String>("title")
        .unwrap()
        .trim()
        .to_ascii_lowercase()
        .replace(' ', "-");
    let output: String = format! {"{}.{}.{}", year, conf, title};

    if let Some(file) = matches.get_one::<PathBuf>("file") {
        let input_path: PathBuf = env::current_dir().unwrap().join(file);
        let output_path: PathBuf = env::current_dir().unwrap().join(format!("{}.pdf", &output));

        if let Err(e) = fs::rename(&input_path, output_path) {
            match e.kind() {
                ErrorKind::NotFound => {
                    eprintln!("[Error] File Not Found, expecting {:?}", input_path)
                }
                _ => eprintln!("[Error] Something Completely Unexpected Happened, {:?}", e),
            }
            return ExitCode::FAILURE;
        }
    } else {
        println!("{}", output);
    };
    ExitCode::SUCCESS
}

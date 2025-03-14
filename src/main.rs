

use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::process;

mod error;
mod wav_header_parser;

fn main() {
    let args: Vec<String> = match env::args().collect() {
        Ok(args) => args,
        Err(err) => {
            eprintln!("Error: Failed to collect command-line arguments - {}", err);
            process::exit(1);
        }
    };

    if args.len() != 2 {
        eprintln!("Error: Invalid number of arguments. Expected 1 argument (file path).");
        process::exit(1);
    }

    let file_path = &args[1];

    if file_path.is_empty() {
        eprintln!("Error: The file path is empty.");
        process::exit(1);
    }

    if !Path::new(file_path).is_file() {
        eprintln!("Error: The provided path is not a file.");
        process::exit(1);
    }

    if !file_path.ends_with(".wav") {
        eprintln!("Error: The file is not a WAV file.");
        process::exit(1);
    }

    match fs::metadata(file_path) {
        Ok(metadata) => {
            if metadata.is_dir() {
                eprintln!("Error: The provided path is a directory.");
                process::exit(1);
            }
            if metadata.len() > 1024 * 1024 * 1024 {
                eprintln!("Error: The file is too large.");
                process::exit(1);
            }
        }
        Err(err) => {
            eprintln!("Error: Failed to get file metadata - {}", err);
            process::exit(1);
        }
    }

    match wav_header_parser::parse_wav_header(file_path) {
        Ok(_) => process::exit(0),
        Err(err) => match err {
            error::Error::InvalidWavHeader => {
                eprintln!("Error: Invalid WAV header.");
                process::exit(1);
            }
            error::Error::IoError(err) => {
                eprintln!("Error: I/O error - {}", err);
                process::exit(1);
            }
            error::Error::InvalidArgument => {
                eprintln!("Error: Invalid argument.");
                process::exit(1);
            }
            error::Error::MetadataError => {
                eprintln!("Error: Failed to retrieve file metadata.");
                process::exit(1);
            }
            error::Error::CorruptedFile => {
                eprintln!("Error: The WAV file is corrupted.");
                process::exit(1);
            }
        },
    }
}
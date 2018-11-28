extern crate spc_parser;

fn main() {
    print_all();
}

use std::ffi::OsString;
use std::fs;

fn print_file(filename: &OsString) {
    let mut raw_file = spc_parser::read_file(filename);

    match spc_parser::parse_file(&mut raw_file) {
        Ok((_, result)) => match result.to_vectors() {
            Ok(result) => result.plot(),
            Err(error) => println!("{}", error),
        },
        Err(_) => println!("Parse error."),
    }
}

fn print_all() {
    println!("");
    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries {
            if let Ok(entry) = entry {
                if entry
                    .file_name()
                    .to_string_lossy()
                    .to_lowercase()
                    .ends_with(".spc")
                {
                    println!("{:?}", entry.file_name());
                    println!("");
                    print_file(&entry.file_name());
                    println!("");
                }
            }
        }
    }
}

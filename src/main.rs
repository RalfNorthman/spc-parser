extern crate spc_parser;

fn main() {
    print_all();
}

use std::ffi::OsString;
use std::fs;

fn print_header(filename: OsString) {
    let mut raw_header = spc_parser::read_header(filename);

    let result = spc_parser::main_header(&mut raw_header);
    {
        println!("{:#?}", result);
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
                    print_header(entry.file_name());
                    println!("");
                }
            }
        }
    }
}

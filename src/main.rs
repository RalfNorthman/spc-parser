extern crate spc_parser;

fn main() {
    print_all();
}

use spc_parser::BoxResult;
use std::ffi::OsString;
use std::fs;

fn print_file(filename: &OsString) -> BoxResult<()> {
    let mut raw_file = spc_parser::read_file(filename)?;

    match spc_parser::parse_file(&mut raw_file) {
        Ok((_, result)) => {
            result.to_vectors()?.wavenumber_to_nm().plot()
        }
        Err(_) => println!("Parse error."),
    }
    Ok(())
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
                    match print_file(&entry.file_name()) {
                        Ok(result) => result,
                        Err(e) => println!("{}", e),
                    }
                    println!("");
                }
            }
        }
    }
}

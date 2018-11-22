extern crate spc_parser;

fn main() {
    print_all();
}

use spc_parser::{FileTypeFlags, Spc};
use std::ffi::OsString;
use std::fs;

fn print_header(filename: OsString) {
    let mut header = spc_parser::read_header(filename);

    if let Ok((
        _,
        Spc {
            file_type_flags:
                FileTypeFlags {
                    z_randomly_ordered,
                    z_not_even,
                    custom_axis_labels,
                    ..
                },
            number_of_subfiles,
            ..
        },
    )) = spc_parser::main_tryout(&mut header)
    {
        println!(
            "multi: {:?}, z_rand: {:?}, not_even: {:?}, \
             # of subfiles: {:?}",
            custom_axis_labels,
            z_not_even,
            z_randomly_ordered,
            number_of_subfiles
        );
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
// z_randomly_ordered: bool,
// z_not_even: bool,
// custom_axis_labels: bool,

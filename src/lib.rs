#[macro_use]
extern crate nom;

use std::fs::File;
use std::io::Read;
use std::ffi::OsString;
use nom::{le_u32, le_f64};

#[derive(Debug, PartialEq)]
pub struct Spc {
    file_type_flags: FileTypeFlags,
    file_version: FileVersion,
    regular_floats: bool,
    number_of_points: u32,
    first_x: f64,
    last_x: f64,
    number_of_subfiles: u32,
}

#[derive(Debug, PartialEq)]
pub struct FileTypeFlags {
    y16bit_precision: bool,
    experiment_extension: bool,
    multifile: bool,
    z_randomly_ordered: bool,
    z_not_even: bool,
    custom_axis_labels: bool,
    each_subfile_own_x_array: bool,
    xy_file: bool,
}

#[derive(Debug, PartialEq)]
pub enum FileVersion {
    NewFormat,
    OldLabCalcFormat,
}

pub fn read_header(filename: OsString) -> [u8; 30] {
    let mut file_handle =
        File::open(filename).expect("Error opening file");

    let mut buf = [0u8; 30];
    file_handle.read(&mut buf).expect("Error reading file");
    buf
}

named!(
    file_version<FileVersion>,
    alt!(
        tag!(&[0x4b]) => { |_| FileVersion::NewFormat } |
        tag!(&[0x4d]) => { |_| FileVersion::OldLabCalcFormat }
        )
);

named!(
    regular_floats<bool>,
    alt!(
        tag!(&[0x80]) => { |_| true } |
        take!(1) => { |_| false }
        )
);

named!(
    bit_to_bool((&[u8], usize)) -> bool,
        alt!(
            tag_bits!(u8, 1, 1) => { |_| true } |
            tag_bits!(u8, 1, 0) => { |_| false }
            )
    );

named!(
    file_type_flags<FileTypeFlags>,
    bits!(
    do_parse!(
        y16bit_precision: bit_to_bool >>
        experiment_extension: bit_to_bool >>
        multifile: bit_to_bool >>
        z_randomly_ordered: bit_to_bool >>
        z_not_even: bit_to_bool >>
        custom_axis_labels: bit_to_bool >>
        each_subfile_own_x_array: bit_to_bool >>
        xy_file: bit_to_bool >>
        (    FileTypeFlags {
                 y16bit_precision,
                 experiment_extension,
                 multifile,
                 z_randomly_ordered,
                 z_not_even,
                 custom_axis_labels,
                 each_subfile_own_x_array,
                 xy_file 
            })
        ))
    );


named!(
    pub main_tryout<Spc>,
    do_parse!(
        file_type_flags: file_type_flags >>
        file_version: file_version >>
        take!(1) >>
        regular_floats: regular_floats >>
        number_of_points: le_u32 >> 
        first_x: le_f64 >>
        last_x: le_f64 >>
        number_of_subfiles: le_u32 >>
        ( Spc{
            file_type_flags,
            file_version,
            regular_floats,
            number_of_points,
            first_x,
            last_x,
            number_of_subfiles,
        })
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_version_test() {
        assert_eq!(
            file_version(&[0x4b]),
            Ok((&[][..], FileVersion::NewFormat))
        );
        assert_eq!(
            file_version(&[0x4d]),
            Ok((&[][..], FileVersion::OldLabCalcFormat))
        );
    }

    #[test]
    fn regular_floats_test() {
        assert_eq!(
            regular_floats(&[0x80]),
            Ok((&[][..], true))
        );
        assert_eq!(
            regular_floats(&[0x3b]),
            Ok((&[][..], false))
        );
    }
}

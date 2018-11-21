#[macro_use]
extern crate nom;

use std::fs::File;
use std::io::Read;

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

pub fn read_header(name: &str) -> [u8; 512] {
    let filename = String::from(name);
    let mut file_handle =
        File::open(filename).expect("Error opening file");

    let mut buf = [0u8; 512];
    file_handle.read(&mut buf).expect("Error reading file");
    buf
}

named!(
    file_version<&[u8], FileVersion>,
    alt!(
        tag!(&[0x4b]) => { |_| FileVersion::NewFormat } |
        tag!(&[0x4d]) => { |_| FileVersion::OldLabCalcFormat }
        )
    );

named!(
    pub just_file_version<FileVersion>,
    do_parse!(
        take!(1) >> file_version: file_version >> (file_version)
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
}

#[macro_use]
extern crate nom;

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

named!(
    bit_to_bool<bool>, 
    bits!( alt!(
        tag_bits!(u8, 1, 0) => { |_| false } | 
        tag_bits!(u8, 1, 1) => { |_| true }
        )
    ));

named!(
    file_type_flags<&[u8], FileTypeFlags>,
    do_parse!(
        y16bit_precision:         bit_to_bool >>
        experiment_extension:     bit_to_bool >>
        multifile:                bit_to_bool >>
        z_randomly_ordered:       bit_to_bool >>
        z_not_even:               bit_to_bool >>
        custom_axis_labels:       bit_to_bool >>
        each_subfile_own_x_array: bit_to_bool >>
        xy_file:                  bit_to_bool >>
        (FileTypeFlags {
            y16bit_precision,
            experiment_extension,
            multifile,
            z_randomly_ordered,
            z_not_even,
            custom_axis_labels,
            each_subfile_own_x_array,
            xy_file,
        })
        )
    );

named!(
    file_version<&[u8], FileVersion>,
    alt!(
        tag!(&[0x4b]) => { |_| FileVersion::NewFormat } |
        tag!(&[0x4d]) => { |_| FileVersion::OldLabCalcFormat }
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
    fn bit_to_bool_test() {
        let bytes = vec![0b01_11_11_11, 0b10_00_00_00];
        let slice_a = &bytes[..];
        let slice_b = &bytes[1..];
        assert_eq!(
            bit_to_bool(slice_a),
            Ok((&slice_a[1..], false))
            );
        assert_eq!(
            bit_to_bool(slice_b),
            Ok((&slice_b[1..], true))
            );
    }

    #[test]
    fn file_type_flags_test() {
        let byte = vec![0b01_10_11_00];
        let slice = &byte[..];
        assert_eq!(
            file_type_flags(slice),
            Ok((&[][..], 
                FileTypeFlags {
                    y16bit_precision: false,
                    experiment_extension: true,
                    multifile: true,
                    z_randomly_ordered: false,
                    z_not_even: true,
                    custom_axis_labels: true,
                    each_subfile_own_x_array: false,
                    xy_file: false,
                }))
            );
    }
}

#[macro_use]
extern crate nom;
extern crate textplots;

use std::fs::File;
use std::io::Read;
use std::ffi::OsString;
use nom::{le_u32, le_f64, le_f32};
use textplots::{Chart, Plot, Shape};

#[derive(Debug, PartialEq)]
pub struct Spc {
    pub file_type_flags: FileTypeFlags,
    pub file_version: FileVersion,
    pub regular_floats: bool,
    pub number_of_points: u32,
    pub first_x: f64,
    pub last_x: f64,
    pub number_of_subfiles: u32,
    pub x_unit: XUnit,
    pub y_unit: YUnit,
    pub z_unit: YUnit,
    pub xy_single_file_x_data: Option<Vec<f32> >,
    pub single_and_xyy_multi_y_data: Option<Vec<f32> >,
}

#[derive(Debug, PartialEq)]
pub struct FileTypeFlags {
    pub y16bit_precision: bool,
    pub experiment_extension: bool,
    pub multifile: bool,
    pub z_randomly_ordered: bool,
    pub z_not_even: bool,
    pub custom_axis_labels: bool,
    pub each_subfile_own_x_array: bool,
    pub xy_file: bool,
}

#[derive(Debug, PartialEq)]
pub enum FileVersion {
    NewFormat,
    OldLabCalcFormat,
}

#[derive(Debug, PartialEq)]
pub enum XUnit {
    Arbitrary,
    WaveNumber,
    MicroMeters,
    NanoMeters,
    Seconds,
    Minutes,
    Hertz,
    KHertz,
    MHertz,
    Mass,
    PartsPerMillion,
    Days,
    Years,
    RamanShift,
    ElectronVolt,
    Custom,
    DiodeNumber,
    Channel,
    Degrees,
    TemperatureF,
    TemperatureC,
    TemperatureK,
    DataPoints,
    MilliSeconds,
    MicroSeconds,
    NanoSeconds,
    GigaHertz,
    Centimeters,
    Meters,
    MilliMeters,
    Hours,
    NoLabels,
}

#[derive(Debug, PartialEq)]
pub enum YUnit {
    Arbitrary,
    Interferogram,
    Absorbance,
    KubelkaMonk,
    Counts,
    Volts,
    Degrees,
    MilliAmpere,
    MilliMeters,
    MilliVolts,
    LogOneOverR,
    Percent,
    Intensity,
    RelativeIntensity,
    Energy,
    Decibel,
    TemperatureF,
    TemperatureC,
    TemperatureK,
    IndexOfRefraction,
    ExtinctionCoefficient,
    Real,
    Imaginary,
    Complex,
    Transmission,
    Reflectance,
    Valley,
    Emission,
}

pub fn read_file(filename: &OsString) -> Vec<u8> {
    let mut file_handle =
        File::open(filename).expect("Error opening file");

    let file_size = file_handle
        .metadata()
        .expect("Error getting metadata")
        .len();

    let mut buffer: Vec<u8> =
        Vec::with_capacity(file_size as usize);

    file_handle
        .read_to_end(&mut buffer)
        .expect("Error reading file");

    buffer
}

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
            xy_file: bit_to_bool >>
            each_subfile_own_x_array: bit_to_bool >>
            custom_axis_labels: bit_to_bool >>
            z_not_even: bit_to_bool >>
            z_randomly_ordered: bit_to_bool >>
            multifile: bit_to_bool >>
            experiment_extension: bit_to_bool >>
            y16bit_precision: bit_to_bool >>
            ( FileTypeFlags {
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
    )
);

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
    x_unit_p<XUnit>,
    alt!(
        tag!(&[0]) => { |_| XUnit::Arbitrary } |
        tag!(&[1]) => { |_| XUnit::WaveNumber } |
        tag!(&[2]) => { |_| XUnit::MicroMeters } |
        tag!(&[3]) => { |_| XUnit::NanoMeters } |
        tag!(&[4]) => { |_| XUnit::Seconds } |
        tag!(&[5]) => { |_| XUnit::Minutes } |
        tag!(&[6]) => { |_| XUnit::Hertz } |
        tag!(&[7]) => { |_| XUnit::KHertz } |
        tag!(&[8]) => { |_| XUnit::MHertz } |
        tag!(&[9]) => { |_| XUnit::Mass } |
        tag!(&[10]) => { |_| XUnit::PartsPerMillion } |
        tag!(&[11]) => { |_| XUnit::Days } |
        tag!(&[12]) => { |_| XUnit::Years } |
        tag!(&[13]) => { |_| XUnit::RamanShift } |
        tag!(&[14]) => { |_| XUnit::ElectronVolt } |
        tag!(&[15]) => { |_| XUnit::Custom } |
        tag!(&[16]) => { |_| XUnit::DiodeNumber } |
        tag!(&[17]) => { |_| XUnit::Channel } |
        tag!(&[18]) => { |_| XUnit::Degrees } |
        tag!(&[19]) => { |_| XUnit::TemperatureF } |
        tag!(&[20]) => { |_| XUnit::TemperatureC } |
        tag!(&[21]) => { |_| XUnit::TemperatureK } |
        tag!(&[22]) => { |_| XUnit::DataPoints } |
        tag!(&[23]) => { |_| XUnit::MilliSeconds } |
        tag!(&[24]) => { |_| XUnit::MicroSeconds } |
        tag!(&[25]) => { |_| XUnit::NanoSeconds } |
        tag!(&[26]) => { |_| XUnit::GigaHertz } |
        tag!(&[27]) => { |_| XUnit::Centimeters } |
        tag!(&[28]) => { |_| XUnit::Meters } |
        tag!(&[29]) => { |_| XUnit::MilliMeters } |
        tag!(&[30]) => { |_| XUnit::Hours } |
        tag!(&[255]) => { |_| XUnit::NoLabels }
        )
);

named!(
    y_unit_p<YUnit>,
    alt!(
        tag!(&[0]) => { |_| YUnit::Arbitrary } |
        tag!(&[1]) => { |_| YUnit::Interferogram } |
        tag!(&[2]) => { |_| YUnit::Absorbance } |
        tag!(&[3]) => { |_| YUnit::KubelkaMonk } |
        tag!(&[4]) => { |_| YUnit::Counts } |
        tag!(&[5]) => { |_| YUnit::Volts } |
        tag!(&[6]) => { |_| YUnit::Degrees } |
        tag!(&[7]) => { |_| YUnit::MilliAmpere } |
        tag!(&[8]) => { |_| YUnit::MilliMeters } |
        tag!(&[9]) => { |_| YUnit::MilliVolts } |
        tag!(&[10]) => { |_| YUnit::LogOneOverR } |
        tag!(&[11]) => { |_| YUnit::Percent } |
        tag!(&[12]) => { |_| YUnit::Intensity } |
        tag!(&[13]) => { |_| YUnit::RelativeIntensity } |
        tag!(&[14]) => { |_| YUnit::Energy } |
        tag!(&[16]) => { |_| YUnit::Decibel } |
        tag!(&[19]) => { |_| YUnit::TemperatureF } |
        tag!(&[20]) => { |_| YUnit::TemperatureC } |
        tag!(&[21]) => { |_| YUnit::TemperatureK } |
        tag!(&[22]) => { |_| YUnit::IndexOfRefraction } |
        tag!(&[23]) => { |_| YUnit::ExtinctionCoefficient } |
        tag!(&[24]) => { |_| YUnit::Real } |
        tag!(&[25]) => { |_| YUnit::Imaginary } |
        tag!(&[26]) => { |_| YUnit::Complex } |
        tag!(&[128]) => { |_| YUnit::Transmission } |
        tag!(&[129]) => { |_| YUnit::Reflectance } |
        tag!(&[130]) => { |_| YUnit::Valley } |
        tag!(&[131]) => { |_| YUnit::Emission }
        )
);

named!(
    pub parse_file<Spc>,
    do_parse!(
        // Main header
        file_type_flags: file_type_flags >>
        file_version: file_version >>
        take!(1) >>
        regular_floats: regular_floats >>
        number_of_points: le_u32 >> 
        first_x: le_f64 >>
        last_x: le_f64 >>
        number_of_subfiles: le_u32 >>
        x_unit: x_unit_p >>
        y_unit: y_unit_p >>
        z_unit: y_unit_p >>
        take!(217 - 28 - 3 + 30 + 265) >>
        // End main header
        xy_single_file_x_data: cond!(
            file_type_flags.xy_file &&
            !file_type_flags.multifile &&
            regular_floats,
            count!(le_f32, number_of_points as usize)) >>
        // First subfile header
        take!(32) >>
        // First subfile Y values
        single_and_xyy_multi_y_data: cond!(
            !file_type_flags.each_subfile_own_x_array &&
              regular_floats,
            count!(le_f32, number_of_points as usize)) >>
        ( Spc {
            file_type_flags,
            file_version,
            regular_floats,
            number_of_points,
            first_x,
            last_x,
            number_of_subfiles,
            x_unit,
            y_unit,
            z_unit,
            xy_single_file_x_data,
            single_and_xyy_multi_y_data,
        })
    )
);

fn create_points(start: f64, stop: f64, n: u32) -> Vec<f32> {
    let x1 = start as f32;
    let xn = stop as f32;
    (0..n)
    .into_iter()
    .map(|x| x1 + x as f32 * (xn - x1) / ((n as f32) - 1.))
    .collect()
}

pub fn plot(spc: Spc) {
    if !spc.regular_floats {
        println!("No plot - only support for IEEE floats.");
    } else {
        if let Some(ys) = spc.single_and_xyy_multi_y_data {
            let points: Vec<(f32, f32)> =
                if let Some(xs) = spc.xy_single_file_x_data {
                    xs.iter().map(|x| *x).zip(ys).collect()
                } else {
                    let xs = create_points(
                        spc.first_x,
                        spc.last_x,
                        spc.number_of_points,
                        );
                    xs.iter().map(|x| *x).zip(ys).collect()
                };
            Chart::new(
                    120,
                    90,
                    spc.first_x as f32,
                    spc.last_x as f32,
                    )
                .lineplot(Shape::Lines(&points))
                .display()
        } else {
            println!("No plot - something went wrong.");
        }
    }
}

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

    #[test]
    fn x_unit_test() {
        assert_eq!(
            x_unit_p(&[13]),
            Ok((&[][..], XUnit::RamanShift))
        );
        assert_eq!(
            x_unit_p(&[255]),
            Ok((&[][..], XUnit::NoLabels))
        );
        assert_eq!(
            x_unit_p(&[1]),
            Ok((&[][..], XUnit::WaveNumber))
        );
        assert_eq!(
            x_unit_p(&[3]),
            Ok((&[][..], XUnit::NanoMeters))
        );
    }

    #[test]
    fn y_unit_test() {
        assert_eq!(
            y_unit_p(&[19]),
            Ok((&[][..], YUnit::TemperatureF))
        );
        assert_eq!(
            y_unit_p(&[0]),
            Ok((&[][..], YUnit::Arbitrary))
        );
        assert_eq!(
            y_unit_p(&[128]),
            Ok((&[][..], YUnit::Transmission))
        );
    }

    #[test]
    fn create_points_test() {
        assert_eq!(
            create_points(4., 5., 3),
            vec![4., 4.5, 5.]
        );
        assert_eq!(
            create_points(11., 14., 4),
            vec![11., 12., 13., 14.]
        );
        assert_eq!(
            create_points(-2., -1., 5),
            vec![-2., -1.75, -1.5, -1.25, -1.]
        );
    }
}

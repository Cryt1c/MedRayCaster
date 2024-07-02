use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use rayon::prelude::*;
use std::fs::read_to_string;
use std::io::BufReader;
use three_d_asset::{Texture3D, TextureData};

#[derive(Debug, PartialEq)]
pub struct Dim {
    pub width: i32,
    pub height: i32,
    pub depth: i32,
}

pub struct Texture {
    pub texture_data: Vec<u8>,
    pub dimensions: Dim,
}

pub struct Volume {
    pub vertex_data: [f32; 24],
    pub indices: [u32; 36],
    pub texture: Texture,
    pub histogram: Vec<f64>,
}

impl Volume {
    pub fn new() -> Self {
        let vertex_data = [
            -0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5, -0.5, 0.5, 0.5, -0.5, -0.5, -0.5, 0.5,
            -0.5, -0.5, 0.5, 0.5, -0.5, -0.5, 0.5, -0.5,
        ];

        let indices = [
            0, 1, 2, 0, 2, 3, // front
            1, 5, 6, 1, 6, 2, // right
            5, 4, 7, 5, 7, 6, // back
            4, 0, 3, 4, 3, 7, // left
            2, 6, 7, 2, 7, 3, // top
            4, 5, 1, 4, 1, 0, // bottom
        ];

        // TODO: Automatically detect file format and use specific loader.
        // let result = Volume::read_vol("examples/assets/Skull.vol");
        let texture = Volume::read_raw("examples/assets/sinus.raw", "examples/assets/sinus.mhd");

        let histogram = Volume::calculate_histogram(&texture.texture_data);

        Volume {
            vertex_data,
            indices,
            texture,
            histogram,
        }
    }

    pub fn read_vol(file_path: &str) -> Texture {
        let texture_3d: Texture3D = three_d_asset::io::load(&[file_path])
            .unwrap()
            .deserialize("")
            .unwrap();
        let width = texture_3d.width as i32;
        let height = texture_3d.height as i32;
        let depth = texture_3d.depth as i32;
        let texture_data = match texture_3d.data {
            TextureData::RU8(data) => data,
            _ => panic!("Expected RU8 texture data format"),
        };

        Texture {
            dimensions: Dim {
                width,
                height,
                depth,
            },
            texture_data,
        }
    }

    pub fn read_raw(file_path: &str, meta_file_path: &str) -> Texture {
        let meta_data = read_to_string(meta_file_path).expect("Unable to read MHD file");
        let dimensions = Volume::parse_meta_data_dim(&meta_data);

        let num_elements = dimensions.height * dimensions.width * dimensions.depth;
        // let file = File::open(file_path).expect("Unable to open RAW file");
        let file: &[u8] = include_bytes!("../examples/assets/sinus.raw");
        let mut reader = BufReader::new(file);

        let mut raw_data = vec![0u16; num_elements as usize];
        reader
            .read_u16_into::<LittleEndian>(&mut raw_data)
            .expect("Unable to read u16 from RAW file");

        let buffer: Vec<u8> = raw_data
            .par_iter()
            .map(|&value| Volume::normalize_hounsfield_units(value))
            .collect();

        Texture {
            dimensions: Dim {
                width: dimensions.width,
                height: dimensions.height,
                depth: dimensions.depth,
            },
            texture_data: buffer,
        }
    }

    pub fn normalize_hounsfield_units(value: u16) -> u8 {
        let normalized_hu_value = (value as f32 / 4095.0) * 255.0; // Normalize to [0, 255]
        normalized_hu_value as u8
    }

    pub fn parse_meta_data_dim(meta_data: &str) -> Dim {
        let mut dimensions = Dim {
            width: 0,
            height: 0,
            depth: 0,
        };

        meta_data.lines().for_each(|line| {
            let mut data = line.split(" = ");

            match data.next() {
                Some("NDims") => {
                    let _value = data.next().unwrap().parse::<i32>().unwrap();
                    // TODO: Use NDims info.
                }
                Some("DimSize") => {
                    let value = data
                        .next()
                        .unwrap()
                        .split(" ")
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>();
                    dimensions.width = value[0];
                    dimensions.height = value[1];
                    dimensions.depth = value[2];
                }
                Some("ElementSpacing") => {
                    let _value = data.next().unwrap();
                    // TODO: Use ElementSpacing info.
                }
                _ => {
                    println!("Unknown field");
                }
            }
        });

        dimensions
    }

    pub fn calculate_histogram(texture_data: &Vec<u8>) -> Vec<f64> {
        let mut histogram = vec![0.0_f64; 256];

        texture_data.iter().for_each(|&value| {
            histogram[value as usize] += 1.0_f64;
        });

        histogram
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hounsfield_normalization() {
        let input: Vec<u16> = vec![4095, 0, 2047];
        let expected: Vec<u8> = vec![255, 0, 127];
        let result: Vec<u8> = input
            .iter()
            .map(|&x| Volume::normalize_hounsfield_units(x))
            .collect();

        assert_eq!(expected, result);
    }

    #[test]
    fn test_parse_dim() {
        let input = "NDims = 3\nDimSize = 512 512 333\nElementSpacing = 0.402344 0.402344 0.899994";
        let expected = Dim {
            width: 512,
            height: 512,
            depth: 333,
        };
        let result = Volume::parse_meta_data_dim(input);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_histogram_calculation() {
        let input: Vec<u8> = vec![0, 0, 0, 128, 128, 128, 255, 255, 255];
        let mut expected: Vec<f64> = vec![0.0; 256];
        expected[0] = 3.0;
        expected[128] = 3.0;
        expected[255] = 3.0;
        let result = Volume::calculate_histogram(&input);

        assert_eq!(expected, result);
    }
}

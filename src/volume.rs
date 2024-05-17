use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use std::fs::File;
use three_d_asset::{Texture3D, TextureData};

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

        // let result = Volume::read_vol("examples/assets/Skull.vol");
        let result = Volume::read_raw(
            "examples/assets/sinus.raw",
            Dim {
                width: 512,
                height: 512,
                depth: 333,
            },
        );

        Volume {
            vertex_data,
            indices,
            texture: result,
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

    pub fn read_raw(file_path: &str, dimensions: Dim) -> Texture {
        let num_elements = dimensions.height * dimensions.width * dimensions.depth;
        let mut file = File::open(file_path).expect("Unable to open RAW file");
        let mut buffer = Vec::with_capacity(num_elements.try_into().unwrap());

        for _ in 0..num_elements {
            let value = file
                .read_u16::<LittleEndian>()
                .expect("Unable to read u16 from RAW file");
            let normalized_hu_value = Volume::normalize_hounsfield_units(value);
            println!("{:?}", normalized_hu_value);
            buffer.push(normalized_hu_value);
        }

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
        let hu_value = value & 0x0FFF; // Use only the lower 12 bits
        let normalized_hu_value = (hu_value as f32 / 4095.0) * 256.0; // Normalize to [0, 256]
        normalized_hu_value as u8
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
        println!("{:?}", result);

        assert_eq!(expected, result);
    }
}

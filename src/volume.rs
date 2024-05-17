use byteorder::{LittleEndian, ReadBytesExt};
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

        let result = Volume::read_vol("examples/assets/Skull.vol");

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
            _ => panic!("Expected RU8 texture data format"), // Handle other cases as needed
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
}

use crate::shader::{Shader, ShaderType};
use crate::volume::Volume;
use glow::{Buffer, HasContext, Texture, VertexArray};
use nalgebra::Matrix4;
use std::{mem, sync::Arc};
use three_d::{degrees, Camera, Context, Viewport};

pub struct Renderer {
    pub gl: Arc<three_d::Context>,
    pub vbo: Option<Buffer>,
    pub vao: Option<VertexArray>,
    pub ebo: Option<Buffer>,
    pub texture: Option<Texture>,
    pub scene: Scene,
}

pub struct Scene {
    pub volume: Volume,
    pub camera: Camera,
    pub shader_type: ShaderType,
    pub lower_threshold: u8,
    pub upper_threshold: u8,
}

pub struct Uniforms {
    pub cam_pos: three_d_asset::Vector3<f32>,
    pub model_matrix: Matrix4<f32>,
    pub view_matrix: three_d_asset::Matrix4<f32>,
    pub projection_matrix: three_d_asset::Matrix4<f32>,
    pub lower_threshold: u8,
    pub upper_threshold: u8,
}

impl Renderer {
    pub fn new(context: Context) -> Self {
        let arc_context = Arc::new(context.clone());

        let camera = Camera::new_perspective(
            Viewport {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            },
            three_d_asset::Vector3::new(0.0, 0.0, -2.5),
            three_d_asset::Vector3::new(0.0, 0.0, 0.0),
            three_d_asset::Vector3::new(0.0, 1.0, 0.0),
            degrees(45.0),
            0.1,
            100.0,
        );

        let mut renderer = Renderer {
            gl: arc_context,
            vao: None,
            vbo: None,
            ebo: None,
            texture: None,
            scene: Scene {
                volume: Volume::new(),
                camera,
                shader_type: ShaderType::DefaultShader,
                lower_threshold: 0,
                upper_threshold: 255,
            },
        };
        renderer.create_vao();
        renderer.create_vbo();
        renderer.create_ebo();
        renderer.create_texture();
        renderer
    }
    pub fn create_vao(&mut self) {
        unsafe {
            self.vao = self.gl.create_vertex_array().ok();
            self.gl.bind_vertex_array(self.vao);
        }
    }
    pub fn create_vbo(&mut self) {
        unsafe {
            self.vbo = self.gl.create_buffer().ok();
            self.gl.bind_buffer(glow::ARRAY_BUFFER, self.vbo);
            self.gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&self.scene.volume.vertex_data),
                glow::STATIC_DRAW,
            );
        }
    }
    pub fn create_ebo(&mut self) {
        unsafe {
            self.ebo = self.gl.create_buffer().ok();
            self.gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, self.ebo);
            self.gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                bytemuck::cast_slice(&self.scene.volume.indices),
                glow::STATIC_DRAW,
            );
            self.gl.vertex_attrib_pointer_f32(
                0,
                3,
                glow::FLOAT,
                false,
                3 * mem::size_of::<f32>() as i32,
                0,
            );
            self.gl.enable_vertex_attrib_array(0);
        }
    }
    pub fn create_texture(&mut self) {
        unsafe {
            self.texture = self.gl.create_texture().ok();
            self.gl.bind_texture(glow::TEXTURE_3D, self.texture);
            self.gl.tex_parameter_i32(
                glow::TEXTURE_3D,
                glow::TEXTURE_MIN_FILTER,
                glow::LINEAR as i32,
            );
            self.gl.tex_parameter_i32(
                glow::TEXTURE_3D,
                glow::TEXTURE_MAG_FILTER,
                glow::LINEAR as i32,
            );
            self.gl.tex_parameter_i32(
                glow::TEXTURE_3D,
                glow::TEXTURE_WRAP_S,
                glow::CLAMP_TO_EDGE as i32,
            );
            self.gl.tex_parameter_i32(
                glow::TEXTURE_3D,
                glow::TEXTURE_WRAP_T,
                glow::CLAMP_TO_EDGE as i32,
            );
            self.gl.tex_parameter_i32(
                glow::TEXTURE_3D,
                glow::TEXTURE_WRAP_R,
                glow::CLAMP_TO_EDGE as i32,
            );

            self.gl.tex_image_3d(
                glow::TEXTURE_3D,
                0,
                glow::R8 as i32,
                self.scene.volume.texture.dimensions.width,
                self.scene.volume.texture.dimensions.height,
                self.scene.volume.texture.dimensions.depth,
                0,
                glow::RED,
                glow::UNSIGNED_BYTE,
                Some(bytemuck::cast_slice(
                    &self.scene.volume.texture.texture_data,
                )),
            );
        }
    }

    pub fn calculate_uniforms(&self) -> Uniforms {
        let cam_pos = *self.scene.camera.position();
        Uniforms {
            cam_pos,
            model_matrix: Matrix4::identity(),
            view_matrix: *self.scene.camera.view(),
            projection_matrix: *self.scene.camera.projection(),
            lower_threshold: self.scene.lower_threshold,
            upper_threshold: self.scene.upper_threshold,
        }
    }

    pub fn set_uniform_values(
        uniforms: &Uniforms,
        context: &glow::Context,
        program: glow::Program,
    ) {
        Shader::set_uniform_value(context, program, "M", uniforms.model_matrix);

        Shader::set_uniform_value(context, program, "cam_pos", uniforms.cam_pos);
        Shader::set_uniform_value(context, program, "V", uniforms.view_matrix);
        Shader::set_uniform_value(context, program, "P", uniforms.projection_matrix);
        Shader::set_uniform_value(
            context,
            program,
            "lower_threshold",
            uniforms.lower_threshold,
        );
        Shader::set_uniform_value(
            context,
            program,
            "upper_threshold",
            uniforms.upper_threshold,
        );
    }
}

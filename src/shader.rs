use crate::uniform::Uniform;
use glow::HasContext;
use std::str;

pub struct Shader {
    vertex: String,
    fragment: String,
}

#[derive(Debug, PartialEq)]
pub enum ShaderType {
    DefaultShader,
    MipShader,
    AipShader,
}

impl Shader {
    pub fn load_from_file(vertex_path: &str, fragment_path: &str) -> Shader {
        Shader {
            vertex: include_str!("../shaders/vertex_shader.glsl").to_string(),
            fragment: include_str!("../shaders/mip_shader.glsl").to_string(),
        }
    }

    pub fn get_vertex(&self) -> &str {
        &self.vertex
    }

    pub fn get_fragment(&self) -> &str {
        &self.fragment
    }

    pub fn delete_shader(&self, gl_glow: &glow::Context, shader: glow::Shader) {
        unsafe {
            gl_glow.delete_shader(shader);
        }
    }

    pub fn compile_shader(
        &self,
        gl_glow: &glow::Context,
        src: &str,
        shader_type: u32,
    ) -> glow::Shader {
        unsafe {
            let shader = gl_glow.create_shader(shader_type).unwrap();
            gl_glow.shader_source(shader, src);
            gl_glow.compile_shader(shader);
            let status = gl_glow.get_shader_compile_status(shader);
            if !status {
                let info_log = gl_glow.get_shader_info_log(shader);
                panic!("{}", info_log);
            }
            shader
        }
    }

    pub fn link_program(
        &self,
        gl_glow: &glow::Context,
        vs: glow::Shader,
        fs: glow::Shader,
    ) -> glow::Program {
        unsafe {
            let program = gl_glow.create_program().expect("Cannot create program");
            gl_glow.attach_shader(program, vs);
            gl_glow.attach_shader(program, fs);
            gl_glow.link_program(program);

            let status = gl_glow.get_program_link_status(program);
            if !status {
                let info_log = gl_glow.get_program_info_log(program);
                panic!("{}", info_log);
            }
            program
        }
    }

    pub fn use_program(&self, gl_glow: &glow::Context, program: glow::Program) {
        unsafe {
            gl_glow.use_program(Some(program));
        }
    }

    pub fn set_uniform_value<T: Uniform>(
        gl_glow: &glow::Context,
        program: glow::Program,
        name: &str,
        value: T,
    ) {
        unsafe {
            let location = gl_glow.get_uniform_location(program, name);
            value.set_uniform(gl_glow, location);
        }
    }
}

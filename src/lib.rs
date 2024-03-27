pub mod shader {
    use glow::HasContext;
    use nalgebra::Matrix3;
    use nalgebra::Matrix4;
    use nalgebra::Vector2;
    use nalgebra::Vector3;
    use nalgebra::Vector4;
    use std::str;

    pub struct Shader {
        vertex: String,
        fragment: String,
        gl_glow: glow::Context,
    }

    impl Shader {
        pub fn new(vertex: &str, fragment: &str, gl_glow: glow::Context) -> Shader {
            Shader {
                vertex: vertex.to_string(),
                fragment: fragment.to_string(),
                gl_glow,
            }
        }
        pub fn load_from_file(
            vertex_path: &str,
            fragment_path: &str,
            gl_glow: glow::Context,
        ) -> Shader {
            Shader {
                vertex: std::fs::read_to_string(vertex_path).unwrap(),
                fragment: std::fs::read_to_string(fragment_path).unwrap(),
                gl_glow,
            }
        }
        pub fn get_vertex(&self) -> &str {
            &self.vertex
        }
        pub fn get_fragment(&self) -> &str {
            &self.fragment
        }
        pub fn delete_shader(&self, shader: glow::NativeShader) {
            unsafe {
                self.gl_glow.delete_shader(shader);
            }
        }
        pub fn delete_program(&self, program: glow::NativeProgram) {
            unsafe {
                self.gl_glow.delete_program(program);
            }
        }

        pub fn compile_shader(&self, src: &str, shader_type: u32) -> glow::NativeShader {
            unsafe {
                let glow_shader_type = match shader_type {
                    gl::VERTEX_SHADER => glow::VERTEX_SHADER,
                    gl::FRAGMENT_SHADER => glow::FRAGMENT_SHADER,
                    _ => panic!("Invalid shader type"),
                };
                let shader = self.gl_glow.create_shader(glow_shader_type).unwrap();
                self.gl_glow.shader_source(shader, src);
                self.gl_glow.compile_shader(shader);
                let status = self.gl_glow.get_shader_compile_status(shader);
                if !status {
                    let info_log = self.gl_glow.get_shader_info_log(shader);
                    panic!("{}", info_log);
                }
                shader
            }
        }
        pub fn link_program(
            &self,
            vs: glow::NativeShader,
            fs: glow::NativeShader,
        ) -> glow::NativeProgram {
            unsafe {
                let program = self
                    .gl_glow
                    .create_program()
                    .expect("Cannot create program");
                self.gl_glow.attach_shader(program, vs);
                self.gl_glow.attach_shader(program, fs);
                self.gl_glow.link_program(program);

                let status = self.gl_glow.get_program_link_status(program);
                if !status {
                    let info_log = self.gl_glow.get_program_info_log(program);
                    panic!("{}", info_log);
                }
                program
            }
        }

        pub fn use_program(&self, program: glow::NativeProgram) {
            unsafe {
                self.gl_glow.use_program(Some(program));
            }
        }

        pub fn set_uniform_value<T: Uniform>(
            &self,
            program: glow::NativeProgram,
            name: &str,
            value: T,
        ) {
            unsafe {
                let location = self.gl_glow.get_uniform_location(program, name);
                value.set_uniform(&self.gl_glow, location);
            }
        }
    }
    pub trait Uniform {
        fn set_uniform(&self, gl_glow: &glow::Context, location: Option<glow::UniformLocation>);
    }

    impl Uniform for f32 {
        fn set_uniform(&self, gl_glow: &glow::Context, location: Option<glow::UniformLocation>) {
            unsafe {
                gl_glow.uniform_1_f32(location.as_ref(), *self);
            }
        }
    }

    impl Uniform for i32 {
        fn set_uniform(&self, gl_glow: &glow::Context, location: Option<glow::UniformLocation>) {
            unsafe {
                gl_glow.uniform_1_i32(location.as_ref(), *self);
            }
        }
    }
    impl Uniform for Matrix4<f32> {
        fn set_uniform(&self, gl_glow: &glow::Context, location: Option<glow::UniformLocation>) {
            unsafe {
                gl_glow.uniform_matrix_4_f32_slice(location.as_ref(), false, self.as_slice());
            }
        }
    }
    impl Uniform for Matrix3<f32> {
        fn set_uniform(&self, gl_glow: &glow::Context, location: Option<glow::UniformLocation>) {
            unsafe {
                gl_glow.uniform_matrix_3_f32_slice(location.as_ref(), false, self.as_slice());
            }
        }
    }
    impl Uniform for Vector2<f32> {
        fn set_uniform(&self, gl_glow: &glow::Context, location: Option<glow::UniformLocation>) {
            unsafe {
                gl_glow.uniform_2_f32(location.as_ref(), self.x, self.y);
            }
        }
    }
    impl Uniform for Vector3<f32> {
        fn set_uniform(&self, gl_glow: &glow::Context, location: Option<glow::UniformLocation>) {
            unsafe {
                gl_glow.uniform_3_f32(location.as_ref(), self.x, self.y, self.z);
            }
        }
    }
    impl Uniform for Vector4<f32> {
        fn set_uniform(&self, gl_glow: &glow::Context, location: Option<glow::UniformLocation>) {
            unsafe {
                gl_glow.uniform_4_f32(location.as_ref(), self.x, self.y, self.z, self.w);
            }
        }
    }
}

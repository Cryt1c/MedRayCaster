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
    }

    impl Shader {
        pub fn new(vertex: &str, fragment: &str) -> Shader {
            Shader {
                vertex: vertex.to_string(),
                fragment: fragment.to_string(),
            }
        }
        pub fn load_from_file(vertex_path: &str, fragment_path: &str) -> Shader {
            Shader {
                vertex: std::fs::read_to_string(vertex_path).unwrap(),
                fragment: std::fs::read_to_string(fragment_path).unwrap(),
            }
        }
        pub fn get_vertex(&self) -> &str {
            &self.vertex
        }
        pub fn get_fragment(&self) -> &str {
            &self.fragment
        }
        pub fn delete_shader(&self, gl_glow: &glow::Context, shader: glow::NativeShader) {
            unsafe {
                gl_glow.delete_shader(shader);
            }
        }
        pub fn delete_program(&self, gl_glow: &glow::Context, program: glow::NativeProgram) {
            unsafe {
                gl_glow.delete_program(program);
            }
        }

        pub fn compile_shader(
            &self,
            gl_glow: &glow::Context,
            src: &str,
            shader_type: u32,
        ) -> glow::NativeShader {
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
            vs: glow::NativeShader,
            fs: glow::NativeShader,
        ) -> glow::NativeProgram {
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

        pub fn use_program(&self, gl_glow: &glow::Context, program: glow::NativeProgram) {
            unsafe {
                gl_glow.use_program(Some(program));
            }
        }

        pub fn set_uniform_value<T: Uniform>(
            &self,
            gl_glow: &glow::Context,
            program: glow::NativeProgram,
            name: &str,
            value: T,
        ) {
            unsafe {
                let location = gl_glow.get_uniform_location(program, name);
                value.set_uniform(gl_glow, location);
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

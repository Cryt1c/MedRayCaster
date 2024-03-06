use gl::types::*;
use glutin::window;
use nalgebra::Matrix4;
use nalgebra::Vector2;
use nalgebra::Vector3;
use nalgebra::Vector4;
use nalgebra_glm::TVec3;
use opengl_rs::shader;
use std::f32::consts::PI;
use std::ffi::CString;
use std::mem;
use std::ptr;
use std::str;
use three_d_asset::Texture3D;
use three_d_asset::TextureData;

static VERTEX_DATA: [GLfloat; 32] = [
    // positions          // colors           // texture coords
    0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, // top right
    0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, // bottom right
    -0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, // bottom let
    -0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, // top left
];

static INDICES: [GLuint; 6] = [0, 1, 3, 1, 2, 3];

fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader;
    unsafe {
        shader = gl::CreateShader(ty);
        // Attempt to compile the shader
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        // Get the compile status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(
                shader,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "{}",
                str::from_utf8(&buf)
                    .ok()
                    .expect("ShaderInfoLog not valid utf8")
            );
        }
    }
    shader
}

fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);
        // Get the link status
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(
                program,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "{}",
                str::from_utf8(&buf)
                    .ok()
                    .expect("ProgramInfoLog not valid utf8")
            );
        }
        program
    }
}

fn load_image(path: &str) -> image::DynamicImage {
    let img = image::open(path).unwrap();
    img
}

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let window = glutin::window::WindowBuilder::new();
    let gl_window = glutin::ContextBuilder::new()
        .build_windowed(window, &event_loop)
        .unwrap();

    // It is essential to make the context current before calling `gl::load_with`.
    let gl_window = unsafe { gl_window.make_current() }.unwrap();

    // Load the OpenGL function pointers
    gl::load_with(|symbol| gl_window.get_proc_address(symbol));

    let shaders = shader::Shader::load_from_file(
        "shaders/vertex_shader.glsl",
        "shaders/fragment_shader.glsl",
    );

    // Create GLSL shaders
    let vs = compile_shader(shaders.get_vertex(), gl::VERTEX_SHADER);
    let fs = compile_shader(shaders.get_fragment(), gl::FRAGMENT_SHADER);
    let program = link_program(vs, fs);

    let mut vao = 0;
    let mut vbo = 0;
    let mut ebo = 0;
    let mut texture = 0;

    let TEXTURE_SIZE = 4456448;
    let mut texture_data: [u8; 4456448] = [255; 4456448];
    // [
    //     // First layer (2x2)
    //     255, 0, 0, 255, // Red pixel
    //     0, 255, 0, 255, // Green pixel
    //     0, 0, 255, 255, // Blue pixel
    //     255, 255, 0, 255, // Yellow pixel
    //     // Second layer (2x2)
    //     255, 0, 255, 255, // Magenta pixel
    //     0, 255, 255, 255, // Cyan pixel
    //     255, 255, 255, 255, // White pixel
    //     0, 0, 0, 255, // Black pixel
    // ];
    let texture_3d: Texture3D = three_d_asset::io::load(&["examples/assets/Skull.vol"])
        .unwrap()
        .deserialize("")
        .unwrap();
    let texture_3d_data: TextureData = texture_3d.data;
    if let TextureData::RU8(data) = texture_3d_data {
        for (i, &value) in data.iter().enumerate().take(TEXTURE_SIZE) {
            texture_data[i] = value;
        }
    }

    unsafe {
        // Create Vertex Array Object
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

        gl::BindVertexArray(vao);

        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (VERTEX_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            mem::transmute(&VERTEX_DATA[0]),
            gl::STATIC_DRAW,
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (INDICES.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            mem::transmute(&INDICES[0]),
            gl::STATIC_DRAW,
        );

        // Specify the layout of the vertex data
        // let pos_attr = gl::GetAttribLocation(program, CString::new("position").unwrap().as_ptr());
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            mem::size_of::<GLfloat>() as GLint * 8,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            mem::size_of::<GLfloat>() as GLint * 8,
            (3 * mem::size_of::<GLfloat>()) as *const _,
        );
        gl::EnableVertexAttribArray(1);

        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            mem::size_of::<GLfloat>() as GLint * 8,
            (6 * mem::size_of::<GLfloat>()) as *const _,
        );
        gl::EnableVertexAttribArray(2);

        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_3D, texture);
        gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as i32);

        gl::TexImage3D(
            gl::TEXTURE_3D,
            0,
            gl::RGB as i32,
            texture_3d.width as i32,
            texture_3d.height as i32,
            texture_3d.depth as i32,
            0,
            gl::RED,
            gl::UNSIGNED_BYTE,
            texture_data.as_ptr() as *const _,
        );
        gl::GenerateMipmap(gl::TEXTURE_3D);
    }

    event_loop.run(move |event, _, control_flow| {
        use glutin::event::{Event, WindowEvent};
        use glutin::event_loop::ControlFlow;
        *control_flow = ControlFlow::Poll;
        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    // Cleanup
                    unsafe {
                        gl::DeleteProgram(program);
                        gl::DeleteShader(fs);
                        gl::DeleteShader(vs);
                        gl::DeleteVertexArrays(1, &vao);
                        gl::DeleteBuffers(1, &vbo);
                        gl::DeleteBuffers(1, &ebo);
                    }
                    *control_flow = ControlFlow::Exit
                }
                _ => (),
            },
            Event::RedrawRequested(_) => {
                unsafe {
                    // Clear the screen to black
                    gl::ClearColor(0.3, 0.3, 0.3, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                    gl::BindTexture(gl::TEXTURE_3D, texture);

                    // Use shader program
                    gl::UseProgram(program);

                    let time = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs_f64();
                    let mut modulated_time = (time * 0.5).sin() * 0.5 + 0.5;
                    // let modulated_time = 0.5;
                    println!("Time: {}", modulated_time);
                    gl::Uniform1d(
                        gl::GetUniformLocation(program, CString::new("time").unwrap().as_ptr()),
                        modulated_time,
                    );
                    // set_uniform_values(program, &gl_window.window());
                    // Draw a triangle from the 3 vertices
                    // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
                    gl::BindVertexArray(vao);
                    gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
                }
                gl_window.swap_buffers().unwrap();
            }
            _ => (),
        }
        gl_window.window().request_redraw();
    });
}
trait Uniform {
    fn set_uniform(&self, location: GLint);
}

impl Uniform for f32 {
    fn set_uniform(&self, location: GLint) {
        unsafe {
            gl::Uniform1f(location, *self);
        }
    }
}

impl Uniform for i32 {
    fn set_uniform(&self, location: GLint) {
        unsafe {
            gl::Uniform1i(location, *self);
        }
    }
}
impl Uniform for Matrix4<f32> {
    fn set_uniform(&self, location: GLint) {
        unsafe {
            gl::UniformMatrix4fv(location, 1, gl::FALSE, self.as_ptr());
        }
    }
}
impl Uniform for Vector2<f32> {
    fn set_uniform(&self, location: GLint) {
        unsafe {
            gl::Uniform2fv(location, 1, self.as_ptr());
        }
    }
}
impl Uniform for Vector3<f32> {
    fn set_uniform(&self, location: GLint) {
        unsafe {
            gl::Uniform3fv(location, 1, self.as_ptr());
        }
    }
}
impl Uniform for Vector4<f32> {
    fn set_uniform(&self, location: GLint) {
        unsafe {
            gl::Uniform4fv(location, 1, self.as_ptr());
        }
    }
}

fn set_uniform_value<T: Uniform>(program: GLuint, name: &str, value: T) {
    unsafe {
        let location = gl::GetUniformLocation(program, CString::new(name).unwrap().as_ptr());
        value.set_uniform(location);
    }
}

fn set_uniform_values(program: GLuint, window: &window::Window) {
    let m_dist_exp = -200.0;
    let m_view_matrix = nalgebra_glm::translate(
        &nalgebra_glm::identity(),
        &TVec3::new(0.0, 0.0, -4.0 * (m_dist_exp / 600.0 as f32).exp()),
    );
    let m_fov: f32 = 60.0;
    let m_aspect_ratio = window.inner_size().width as f32 / window.inner_size().height as f32;
    let fov_radians = m_fov.to_radians();
    let m_model_view_projection_matrix =
        nalgebra_glm::perspective(m_aspect_ratio, fov_radians, 0.1, 100.0);
    // TODO:
    // let m_normal_matrix: Matrix4<f32> =
    //     m_view_matrix * m_raycasting_volume.model_matrix.normal_matrix();
    let m_focal_length = 1.0 / ((PI / 180.0 * m_fov / 2.0).tan());
    let m_viewport_size = Vector2::new(
        window.inner_size().width as f32,
        window.inner_size().height as f32,
    );
    // Was Vec3 in original code
    let m_ray_origin = m_view_matrix.try_inverse().unwrap() * Vector4::new(0.0, 0.0, 0.0, 1.0);
    // TODO: fix top and bottom
    let top = m_focal_length * (m_viewport_size.y / 2.0);
    let bottom = -top;
    let m_background = Vector3::new(0.0, 0.0, 0.0);
    let m_light_position = Vector3::new(3.0, 0.0, 3.0);
    let m_diffuse_material = Vector3::new(1.0, 1.0, 1.0);
    // Slider in original code
    let m_step_length = 0.5;
    // Slider in original code
    let m_threshold = 0.1;
    let m_gamma = 2.2;
    let m_volume = 0.0;
    let jitter = 1.0;

    set_uniform_value(program, "m_view_matrix", m_view_matrix);
    set_uniform_value(
        program,
        "m_model_view_projection_matrix",
        m_model_view_projection_matrix,
    );
    set_uniform_value(
        program,
        "ModelViewProjectionMatrix",
        m_model_view_projection_matrix,
    );
    // TODO:
    // set_uniform_value(program, "NormalMatrix", m_normal_matrix);
    set_uniform_value(program, "aspect_ratio", m_aspect_ratio);
    set_uniform_value(program, "focal_length", m_focal_length);
    set_uniform_value(program, "viewport_size", m_viewport_size);
    set_uniform_value(program, "ray_origin", m_ray_origin);
    set_uniform_value(program, "top", top);
    set_uniform_value(program, "bottom", bottom);
    set_uniform_value(program, "background_colour", m_background);
    set_uniform_value(program, "light_position", m_light_position);
    set_uniform_value(program, "material_colour", m_diffuse_material);
    set_uniform_value(program, "step_length", m_step_length);
    set_uniform_value(program, "threshold", m_threshold);
    set_uniform_value(program, "gamma", m_gamma);
    set_uniform_value(program, "volume", m_volume);
    set_uniform_value(program, "jitter", jitter);
}

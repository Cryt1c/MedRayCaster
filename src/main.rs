use gl::types::*;
use glutin::window;
use nalgebra::Matrix3;
use nalgebra::Matrix4;
use nalgebra::Vector2;
use nalgebra::Vector3;
use nalgebra::Vector4;
use opengl_rs::shader;
use std::ffi::CString;
use std::mem;
use std::str;
use three_d_asset::Texture3D;
use three_d_asset::TextureData;

static VERTEX_DATA: [GLfloat; 24] = [
    -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, -1.0, -1.0, 1.0, -1.0,
    -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0,
];

static INDICES: [GLuint; 36] = [
    // front
    0, 1, 2, 0, 2, 3, // right
    1, 5, 6, 1, 6, 2, // back
    5, 4, 7, 5, 7, 6, // left
    4, 0, 3, 4, 3, 7, // top
    2, 6, 7, 2, 7, 3, // bottom
    4, 5, 1, 4, 1, 0,
];

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

    let shaders =
        shader::Shader::load_from_file("shaders/vertex_shader.glsl", "shaders/mip_shader.glsl");

    // Create GLSL shaders
    let vs = shader::Shader::compile_shader(shaders.get_vertex(), gl::VERTEX_SHADER);
    let fs = shader::Shader::compile_shader(shaders.get_fragment(), gl::FRAGMENT_SHADER);
    let program = shader::Shader::link_program(vs, fs);

    let mut vao = 0;
    let mut vbo = 0;
    let mut ebo = 0;
    let mut texture = 0;

    let texture_size = 4456448;
    let mut texture_data: [u8; 4456448] = [255; 4456448];

    let texture_3d: Texture3D = three_d_asset::io::load(&["examples/assets/Skull.vol"])
        .unwrap()
        .deserialize("")
        .unwrap();
    let texture_3d_data: TextureData = texture_3d.data;
    if let TextureData::RU8(data) = texture_3d_data {
        for (i, &value) in data.iter().enumerate().take(texture_size) {
            texture_data[i] = value;
        }
    }
    println!("Max: {}", texture_data.iter().max().unwrap());
    println!("Min: {}", texture_data.iter().min().unwrap());

    unsafe {
        // Create Vertex Array Object

        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (VERTEX_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            mem::transmute(&VERTEX_DATA[0]),
            gl::STATIC_DRAW,
        );

        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (INDICES.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            mem::transmute(&INDICES[0]),
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            mem::size_of::<GLfloat>() as GLint * 3,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        println!("Width: {}", texture_3d.width);
        println!("Height: {}", texture_3d.height);
        println!("Depth: {}", texture_3d.depth);

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
                    gl::ClearColor(1.0, 0.0, 0.0, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                    gl::BindTexture(gl::TEXTURE_3D, texture);

                    // Use shader program
                    gl::UseProgram(program);

                    set_uniform_values(program, &gl_window.window());
                    // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
                    gl::BindVertexArray(vao);
                    gl::DrawElements(
                        gl::TRIANGLES,
                        INDICES.len().try_into().unwrap(),
                        gl::UNSIGNED_INT,
                        std::ptr::null(),
                    );
                    if gl::GetError() != gl::NO_ERROR {
                        println!("Error: {}", gl::GetError());
                    }
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
            gl::UniformMatrix4fv(location, 1, gl::FALSE, self.as_slice().as_ptr());
        }
    }
}
impl Uniform for Matrix3<f32> {
    fn set_uniform(&self, location: GLint) {
        unsafe {
            gl::UniformMatrix3fv(location, 1, gl::FALSE, self.as_slice().as_ptr());
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
        if location == -1 {
            println!("Uniform {} not found", name);
        }
        value.set_uniform(location);
    }
}

fn set_uniform_values(program: GLuint, window: &window::Window) {
    let m_fov: f32 = 60.0;
    let fov_radians = m_fov.to_radians();
    let m_aspect_ratio = window.inner_size().width as f32 / window.inner_size().height as f32;
    let time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
        .sin()
        .abs() as f32;
    

    let mut m_model_view_projection_matrix =
        nalgebra_glm::perspective(fov_radians, m_aspect_ratio, 0.1, 100.0);

    m_model_view_projection_matrix = nalgebra_glm::translate(
        &m_model_view_projection_matrix,
        &Vector3::new(0.0, 0.0, 0.0),
    );
    set_uniform_value(
        program,
        "m_model_view_projection_matrix",
        m_model_view_projection_matrix,
    );
    let viewport_size = Vector2::new(
        window.inner_size().width as f32,
        window.inner_size().height as f32,
    );
    set_uniform_value(program, "viewport_size", viewport_size);
}

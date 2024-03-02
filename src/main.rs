extern crate gl;
extern crate glutin;

use gl::types::*;
use std::ffi::CString;
use std::mem;
use std::ptr;
use std::str;

// Vertex data
// static VERTEX_DATA: [GLfloat; 18] = [
//     // first triangle
//     0.5, 0.5, 0.0, // top right
//     0.5, -0.5, 0.0, // bottom right
//     -0.5, 0.5, 0.0, // top left
//     // second triangle
//     0.5, -0.5, 0.0, // bottom right
//     -0.5, -0.5, 0.0, // bottom left
//     -0.5, 0.5, 0.0, // top left];
// ];

static VERTEX_DATA: [GLfloat; 12] = [
    0.5, 0.5, 0.0, // top right
    0.5, -0.5, 0.0, // bottom right
    -0.5, -0.5, 0.0, // bottom left
    -0.5, 0.5, 0.0, // top left
];

static INDICES: [GLint; 6] = [
    0, 1, 3, // first triangle
    1, 2, 3, // second triangle
];

// static VERTEX_DATA: [GLfloat; 18] = [// positions         // colors
//     0.5, -0.5, 0.0,  1.0, 0.0, 0.0,   // bottom right
//     -0.5, -0.5, 0.0,  0.0, 1.0, 0.0,   // bottom let
//     0.0,  0.5, 0.0,  0.0, 0.0, 1.0    // top
// ];

// Shader sources
static VS_SRC: &'static str = "
#version 330
layout (location = 0) in vec3 position;

void main() {
    gl_Position = vec4(position, 1.0);
}";

static FS_SRC: &'static str = "
#version 330
out vec4 out_color;

uniform vec4 ourColor;

void main() {
    out_color = ourColor;
}";

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

    // Create GLSL shaders
    let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
    let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
    let program = link_program(vs, fs);

    let mut vao = 0;
    let mut vbo = 0;
    let mut ebo = 0;

    unsafe {
        // Create Vertex Array Object
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (VERTEX_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            mem::transmute(&VERTEX_DATA[0]),
            gl::STATIC_DRAW,
        );

        // Created an Element Buffer Object
        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (INDICES.len() * mem::size_of::<GLint>()) as GLsizeiptr,
            mem::transmute(&INDICES[0]),
            gl::STATIC_DRAW,
        );

        // Use shader program
        gl::UseProgram(program);
        gl::BindFragDataLocation(program, 0, CString::new("out_color").unwrap().as_ptr());

        // Specify the layout of the vertex data
        // let pos_attr = gl::GetAttribLocation(program, CString::new("position").unwrap().as_ptr());
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            mem::size_of::<GLfloat>() as GLint * 3,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
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
                        gl::DeleteBuffers(1, &vbo);
                        gl::DeleteVertexArrays(1, &vao);
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
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs_f64();
                    let green_value = (now.sin() / 2.0) + 0.5;
                    let vertex_color_location =
                        gl::GetUniformLocation(program, CString::new("ourColor").unwrap().as_ptr());
                    gl::Uniform4f(vertex_color_location, 0.0, green_value as f32, 0.0, 1.0);
                    // Draw a triangle from the 3 vertices
                    // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
                    gl::DrawElements(
                        gl::TRIANGLES,
                        INDICES.len() as i32,
                        gl::UNSIGNED_INT,
                        ptr::null(),
                    );
                }
                gl_window.swap_buffers().unwrap();
            }
            _ => (),
        }
        gl_window.window().request_redraw();
    });
}

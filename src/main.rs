use gl::types::*;
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
use winit::window;

static VERTEX_DATA: [GLfloat; 24] = [
    -0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5, -0.5, 0.5, 0.5, -0.5, -0.5, -0.5, 0.5, -0.5,
    -0.5, 0.5, 0.5, -0.5, -0.5, 0.5, -0.5,
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
    let (gl, gl_surface, gl_context, shader_version, _window, event_loop) = {
        use glutin::{
            config::{ConfigTemplateBuilder, GlConfig},
            context::{ContextApi, ContextAttributesBuilder, NotCurrentGlContext},
            display::{GetGlDisplay, GlDisplay},
            surface::{GlSurface, SwapInterval},
        };
        use glutin_winit::{DisplayBuilder, GlWindow};
        use raw_window_handle::HasRawWindowHandle;
        use std::num::NonZeroU32;

        let event_loop = winit::event_loop::EventLoopBuilder::new().build().unwrap();
        let window_builder = winit::window::WindowBuilder::new()
            .with_title("Hello triangle!")
            .with_inner_size(winit::dpi::LogicalSize::new(1024.0, 768.0));

        let template = ConfigTemplateBuilder::new();

        let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));

        let (window, gl_config) = display_builder
            .build(&event_loop, template, |configs| {
                configs
                    .reduce(|accum, config| {
                        if config.num_samples() > accum.num_samples() {
                            config
                        } else {
                            accum
                        }
                    })
                    .unwrap()
            })
            .unwrap();

        let raw_window_handle = window
            .as_ref()
            .map(|window| window.raw_window_handle())
            .unwrap();

        let gl_display = gl_config.display();
        let context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(glutin::context::Version {
                major: 4,
                minor: 3,
            })))
            .build(Some(raw_window_handle));
        unsafe {
            let not_current_gl_context = gl_display
                .create_context(&gl_config, &context_attributes)
                .unwrap();

            let window = window.unwrap();

            let attrs = window.build_surface_attributes(Default::default());
            let gl_surface = gl_display
                .create_window_surface(&gl_config, &attrs)
                .unwrap();

            let gl_context = not_current_gl_context.make_current(&gl_surface).unwrap();

            let gl = glow::Context::from_loader_function_cstr(|s| gl_display.get_proc_address(s));
            gl::load_with(|symbol| gl_display.get_proc_address(&CString::new(symbol).unwrap()));

            gl_surface
                .set_swap_interval(&gl_context, SwapInterval::Wait(NonZeroU32::new(1).unwrap()))
                .unwrap();

            (
                gl,
                gl_surface,
                gl_context,
                "#version 410",
                window,
                event_loop,
            )
        }
    };
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

    let _ = event_loop.run(move |event, elwt| {
        use glutin::prelude::GlSurface;
        use winit::event::{Event, WindowEvent};
        match event {
            Event::LoopExiting => return,
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
                    elwt.exit();
                }
                WindowEvent::RedrawRequested => {
                    unsafe {
                        // Clear the screen to black
                        gl::ClearColor(1.0, 0.0, 0.0, 1.0);
                        gl::Clear(gl::COLOR_BUFFER_BIT);
                        gl::BindTexture(gl::TEXTURE_3D, texture);

                        // Use shader program
                        gl::UseProgram(program);

                        set_uniform_values(program, &_window);
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
                    gl_surface.swap_buffers(&gl_context).unwrap();
                }
                _ => (),
            },
            _ => (),
        }
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
    let m_fov: f32 = 45.0;
    let fov_radians = m_fov.to_radians();
    let m_aspect_ratio = window.inner_size().width as f32 / window.inner_size().height as f32;
    let time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();
    let time_sin = time.sin() as f32;

    let model_matrix =
        nalgebra_glm::rotate(&Matrix4::identity(), 0.0, &Vector3::new(0.0, 1.0, 0.0));
    let mut cam_pos = Vector3::new(time_sin, 0.0, -2.0);
    let view_matrix = nalgebra_glm::translate(&Matrix4::identity(), &cam_pos);

    let projection_matrix = nalgebra_glm::perspective(fov_radians, m_aspect_ratio, 0.1, 100.0);

    set_uniform_value(program, "camPos", cam_pos);
    set_uniform_value(program, "M", model_matrix);
    set_uniform_value(program, "V", view_matrix);
    set_uniform_value(program, "P", projection_matrix);
}

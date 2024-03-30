mod renderer;

use glow::HasContext;
use glow::NativeProgram;
use glutin::surface::GlSurface;
use nalgebra::Matrix4;
use nalgebra::Vector3;
use opengl_rs::shader;
use std::mem;
use three_d_asset::Texture3D;
use three_d_asset::TextureData;
use winit::event::{Event, WindowEvent};
use winit::window;

static VERTEX_DATA: [f32; 24] = [
    -0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5, -0.5, 0.5, 0.5, -0.5, -0.5, -0.5, 0.5, -0.5,
    -0.5, 0.5, 0.5, -0.5, -0.5, 0.5, -0.5,
];

static INDICES: [u32; 36] = [
    // front
    0, 1, 2, 0, 2, 3, // right
    1, 5, 6, 1, 6, 2, // back
    5, 4, 7, 5, 7, 6, // left
    4, 0, 3, 4, 3, 7, // top
    2, 6, 7, 2, 7, 3, // bottom
    4, 5, 1, 4, 1, 0,
];

fn main() {
    let renderer = renderer::Renderer::new();
    let event_loop = renderer.event_loop;
    let gl_window = renderer.gl_window;
    let gl_surface = renderer.gl_surface;
    let gl_glow = renderer.gl_glow;
    let gl_possibly_current_context = renderer.gl_possibly_current_context;

    let shaders =
        shader::Shader::load_from_file("shaders/vertex_shader.glsl", "shaders/raycaster.glsl");
    // Create GLSL shaders
    let vs = shaders.compile_shader(&gl_glow, shaders.get_vertex(), glow::VERTEX_SHADER);
    let fs = shaders.compile_shader(&gl_glow, shaders.get_fragment(), glow::FRAGMENT_SHADER);
    let program = shaders.link_program(&gl_glow, vs, fs);

    let texture = unsafe { gl_glow.create_texture().expect("Cannot create texture") };
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

    let vao;
    let vbo;
    let ebo;
    unsafe {
        // Create Vertex Array Object
        vao = gl_glow.create_vertex_array().expect("Cannot create VAO");
        gl_glow.bind_vertex_array(Some(vao));

        vbo = gl_glow.create_buffer().expect("Cannot create VBO");
        gl_glow.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl_glow.buffer_data_u8_slice(
            glow::ARRAY_BUFFER,
            bytemuck::cast_slice(&VERTEX_DATA),
            glow::STATIC_DRAW,
        );

        ebo = gl_glow.create_buffer().expect("Cannot create EBO");
        gl_glow.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
        gl_glow.buffer_data_u8_slice(
            glow::ELEMENT_ARRAY_BUFFER,
            bytemuck::cast_slice(&INDICES),
            glow::STATIC_DRAW,
        );
        gl_glow.vertex_attrib_pointer_f32(
            0,
            3,
            glow::FLOAT,
            false,
            3 * mem::size_of::<f32>() as i32,
            0,
        );
        gl_glow.enable_vertex_attrib_array(0);

        gl_glow.bind_texture(glow::TEXTURE_3D, Some(texture));
        gl_glow.tex_parameter_i32(
            glow::TEXTURE_3D,
            glow::TEXTURE_MIN_FILTER,
            glow::LINEAR as i32,
        );
        gl_glow.tex_parameter_i32(
            glow::TEXTURE_3D,
            glow::TEXTURE_MAG_FILTER,
            glow::LINEAR as i32,
        );
        gl_glow.tex_parameter_i32(
            glow::TEXTURE_3D,
            glow::TEXTURE_WRAP_S,
            glow::CLAMP_TO_EDGE as i32,
        );
        gl_glow.tex_parameter_i32(
            glow::TEXTURE_3D,
            glow::TEXTURE_WRAP_T,
            glow::CLAMP_TO_EDGE as i32,
        );
        gl_glow.tex_parameter_i32(
            glow::TEXTURE_3D,
            glow::TEXTURE_WRAP_R,
            glow::CLAMP_TO_EDGE as i32,
        );
        gl_glow.tex_image_3d(
            glow::TEXTURE_3D,
            0,
            glow::RGB as i32,
            texture_3d.width as i32,
            texture_3d.height as i32,
            texture_3d.depth as i32,
            0,
            glow::RED,
            glow::UNSIGNED_BYTE,
            Some(bytemuck::cast_slice(&texture_data)),
        );
        gl_glow.generate_mipmap(glow::TEXTURE_3D);
    }

    let _ = event_loop.run(move |event, elwt| {
        match event {
            Event::LoopExiting => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    // Cleanup
                    unsafe {
                        shaders.delete_program(&gl_glow, program);
                        shaders.delete_shader(&gl_glow, fs);
                        shaders.delete_shader(&gl_glow, vs);
                        gl_glow.delete_vertex_array(vao);
                        gl_glow.delete_buffer(vbo);
                        gl_glow.delete_buffer(ebo);
                    }
                    elwt.exit();
                }
                WindowEvent::RedrawRequested => {
                    unsafe {
                        // Clear the screen to black
                        gl_glow.clear_color(0.0, 0.0, 0.0, 1.0);
                        gl_glow.clear(glow::COLOR_BUFFER_BIT);
                        gl_glow.bind_texture(glow::TEXTURE_3D, Some(texture));

                        // Use shader program
                        shaders.use_program(&gl_glow, program);

                        set_uniform_values(&gl_glow, program, &gl_window, &shaders);
                        gl_glow.bind_vertex_array(Some(vao));
                        gl_glow.draw_elements(
                            glow::TRIANGLES,
                            INDICES.len() as i32,
                            glow::UNSIGNED_INT,
                            0,
                        );
                        if gl_glow.get_error() != glow::NO_ERROR {
                            println!("Error: {}", gl_glow.get_error());
                        }
                    }
                    gl_surface
                        .swap_buffers(&gl_possibly_current_context)
                        .unwrap();
                }
                _ => (),
            },
            _ => (),
        }
        gl_window.request_redraw();
    });
}

fn set_uniform_values(
    gl_glow: &glow::Context,
    program: NativeProgram,
    window: &window::Window,
    shaders: &shader::Shader,
) {
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
    let cam_pos = Vector3::new(time_sin * 0.5, 0.0, -2.5);
    let view_matrix = nalgebra_glm::translate(&Matrix4::identity(), &cam_pos);

    let projection_matrix = nalgebra_glm::perspective(fov_radians, m_aspect_ratio, 0.1, 100.0);

    shaders.set_uniform_value(&gl_glow, program, "camPos", cam_pos);
    shaders.set_uniform_value(&gl_glow, program, "M", model_matrix);
    shaders.set_uniform_value(&gl_glow, program, "V", view_matrix);
    shaders.set_uniform_value(&gl_glow, program, "P", projection_matrix);
}

mod renderer;
mod volume;

use glow::HasContext;
use glow::NativeProgram;
use glutin::surface::GlSurface;
use nalgebra::Matrix4;
use nalgebra::Vector3;
use opengl_rs::shader;
use winit::event::{Event, WindowEvent};
use winit::window;

fn main() {
    let mut renderer = renderer::Renderer::new();
    let volume = volume::Volume::new();
    renderer.create_vao();
    renderer.create_vbo(&volume);
    renderer.create_ebo(&volume);
    renderer.create_texture(&volume);

    let gl_glow = renderer.gl_glow;
    let gl_possibly_current_context = renderer.gl_possibly_current_context;
    let event_loop = renderer.event_loop;
    let gl_window = renderer.gl_window;
    let gl_surface = renderer.gl_surface;

    let shaders =
        shader::Shader::load_from_file("shaders/vertex_shader.glsl", "shaders/raycaster.glsl");
    // Create GLSL shaders
    let vs = shaders.compile_shader(&gl_glow, shaders.get_vertex(), glow::VERTEX_SHADER);
    let fs = shaders.compile_shader(&gl_glow, shaders.get_fragment(), glow::FRAGMENT_SHADER);
    let program = shaders.link_program(&gl_glow, vs, fs);

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
                        gl_glow.delete_vertex_array(renderer.vao.unwrap());
                        gl_glow.delete_buffer(renderer.vbo.unwrap());
                        gl_glow.delete_buffer(renderer.ebo.unwrap());
                    }
                    elwt.exit();
                }
                WindowEvent::RedrawRequested => {
                    unsafe {
                        // Clear the screen to black
                        gl_glow.clear_color(0.0, 0.0, 0.0, 1.0);
                        gl_glow.clear(glow::COLOR_BUFFER_BIT);
                        gl_glow.bind_texture(glow::TEXTURE_3D, renderer.texture);

                        // Use shader program
                        shaders.use_program(&gl_glow, program);

                        set_uniform_values(&gl_glow, program, &gl_window, &shaders);
                        gl_glow.bind_vertex_array(renderer.vao);
                        gl_glow.draw_elements(
                            glow::TRIANGLES,
                            volume.indices.len() as i32,
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

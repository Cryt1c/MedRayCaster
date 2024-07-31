#![allow(special_module_name)]
pub mod renderer;
pub mod shader;
pub mod ui;
pub mod uniform;
pub mod volume;

use crate::renderer::Renderer;
use crate::shader::Shader;
use crate::shader::ShaderType;
use crate::ui::UserInterface;
use three_d::*;

// Entry point for wasm
use wasm_bindgen::prelude::*;

fn init_wasm() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Debug).unwrap();

    use log::info;
    info!("Logging works!");
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    #[cfg(target_arch = "wasm32")]
    init_wasm();

    let window = Window::new(WindowSettings {
        title: "MedRayCast".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();

    let context = window.gl();
    let mut renderer = Renderer::new(context);

    let mut gui = three_d::GUI::new(&renderer.gl.clone());
    window.render_loop(move |mut frame_input| {
        let mut panel_width = 0.0;
        gui.update(
            &mut frame_input.events,
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |gui_context| {
                use three_d::egui::*;
                egui::CentralPanel::default().show(gui_context, |ui| {
                    ui.vertical(|ui| {
                        UserInterface::render_controls(ui, &mut renderer.scene);
                        UserInterface::render_histogram(ui, &renderer.scene.volume);
                    });
                    if gui_context.input(|i| i.zoom_delta() != 1.0) {
                        renderer.scene.camera.location.z +=
                            gui_context.input(|i| (i.zoom_delta() - 1.0));
                    }
                    egui::Frame::canvas(&Style {
                        visuals: Visuals::dark(),
                        ..Style::default()
                    })
                    .show(ui, |ui| {
                        let available_width = ui.available_width();
                        let available_height = ui.available_height();
                        let available_size = if available_width > available_height {
                            available_height
                        } else {
                            available_width
                        };
                        let (rect, _) = ui.allocate_exact_size(
                            egui::Vec2::new(available_size, available_size),
                            egui::Sense::drag(),
                        );
                        renderer.scene.camera.aspect_ratio = rect.aspect_ratio();

                        // Create local variables to ensure thread safety.
                        let texture = renderer.texture;
                        let vao = renderer.vao;
                        let indices_length = renderer.scene.volume.indices.len();
                        let uniforms = renderer.calculate_uniforms();

                        let fragment_shader = match renderer.scene.shader_type {
                            ShaderType::DefaultShader => "shaders/cookbook_shader.glsl",
                            ShaderType::MipShader => "shaders/mip_shader.glsl",
                            ShaderType::AipShader => "shaders/aip_shader.glsl",
                        };

                        let shaders =
                            Shader::load_from_file("shaders/vertex_shader.glsl", fragment_shader);

                        let callback = egui::PaintCallback {
                            rect,
                            callback: std::sync::Arc::new(egui_glow::CallbackFn::new(
                                move |_info, painter| {
                                    let vs = shaders.compile_shader(
                                        painter.gl(),
                                        shaders.get_vertex(),
                                        glow::VERTEX_SHADER,
                                    );
                                    let fs = shaders.compile_shader(
                                        painter.gl(),
                                        shaders.get_fragment(),
                                        glow::FRAGMENT_SHADER,
                                    );
                                    let program = shaders.link_program(painter.gl(), vs, fs);
                                    shaders.delete_shader(painter.gl(), vs);
                                    shaders.delete_shader(painter.gl(), fs);
                                    shaders.use_program(painter.gl(), program);
                                    Renderer::set_uniform_values(&uniforms, &painter, program);

                                    unsafe {
                                        painter.gl().bind_texture(glow::TEXTURE_3D, texture);
                                        painter.gl().bind_vertex_array(vao);
                                        painter.gl().draw_elements(
                                            glow::TRIANGLES,
                                            indices_length as i32,
                                            glow::UNSIGNED_INT,
                                            0,
                                        );
                                        if painter.gl().get_error() != glow::NO_ERROR {
                                            println!("Error: {}", painter.gl().get_error());
                                        }
                                    }
                                },
                            )),
                        };
                        ui.painter().add(callback);
                    });
                });
                panel_width = gui_context.used_rect().width();
            },
        );

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(1.0, 0.0, 0.0, 1.0, 1.0))
            .write(|| gui.render())
            .unwrap();
        FrameOutput::default()
    });
    Ok(())
}

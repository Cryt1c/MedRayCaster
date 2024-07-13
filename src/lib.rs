#![allow(special_module_name)]
pub mod renderer;
pub mod shader;
pub mod ui;
pub mod uniform;
pub mod volume;

use crate::renderer::Renderer;
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
    let renderer = Renderer::new(context);
    let camera = Camera::new_perspective(
        window.viewport(),
        vec3(0.0, 0.0, 1.3),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        10.0,
    );

    let mut gui = three_d::GUI::new(&renderer.gl.clone());
    let mut viewport_zoom = 1.0;
    let mut scissor_zoom = 1.0;
    window.render_loop(move |mut frame_input| {
        let mut panel_width = 0.0;
        gui.update(
            &mut frame_input.events,
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |gui_context| {
                use three_d::egui::*;
                SidePanel::left("side_panel").show(gui_context, |ui| {
                    use three_d::egui::*;
                    ui.heading("Debug Panel");
                    ui.add(Slider::new(&mut viewport_zoom, 0.01..=1.0).text("Viewport"));
                    ui.add(Slider::new(&mut scissor_zoom, 0.01..=1.0).text("Scissor"));
                });
                panel_width = gui_context.used_rect().width();
            },
        );

        let viewport = Viewport {
            x: (panel_width * frame_input.device_pixel_ratio) as i32,
            y: 0,
            width: frame_input.viewport.width
                - (panel_width * frame_input.device_pixel_ratio) as u32,
            height: frame_input.viewport.height,
        };

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(1.0, 0.0, 0.0, 1.0, 1.0))
            .write(|| gui.render())
            .unwrap();
        //
        // // Secondary view
        // let secondary_viewport = Viewport {
        //     x: viewport.x,
        //     y: viewport.y,
        //     width: 200,
        //     height: 200,
        // };
        // camera.set_viewport(secondary_viewport);
        // frame_input.screen().clear_partially(
        //     secondary_viewport.into(),
        //     ClearState::color_and_depth(0.0, 1.0, 0.0, 1.0, 1.0),
        // );

        // Returns default frame output to end the frame
        FrameOutput::default()
    });
    Ok(())
}
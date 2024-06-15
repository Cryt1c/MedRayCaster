mod renderer;
mod shader;
mod ui;
mod uniform;
mod volume;

use three_d::*;

fn main() {
    // let options = eframe::NativeOptions {
    //     viewport: egui::ViewportBuilder::default().with_inner_size([500.0, 500.0]),
    //     multisampling: 4,
    //     renderer: eframe::Renderer::Glow,
    //     ..Default::default()
    // };
    // let _ = eframe::run_native(
    //     "Raycaster",
    //     options,
    //     Box::new(|cc| Box::new(renderer::Renderer::new(cc))),
    // );
    //

    let window = Window::new(WindowSettings {
        title: "MedRayCast".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();

    let context = window.gl();
    let renderer = renderer::Renderer::new(context);
    let mut camera = Camera::new_perspective(
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

        // Secondary view
        let secondary_viewport = Viewport {
            x: viewport.x,
            y: viewport.y,
            width: 200,
            height: 200,
        };
        camera.set_viewport(secondary_viewport);
        frame_input.screen().clear_partially(
            secondary_viewport.into(),
            ClearState::color_and_depth(0.0, 0.0, 0.0, 1.0, 1.0),
        );

        // Returns default frame output to end the frame
        FrameOutput::default()
    });
}

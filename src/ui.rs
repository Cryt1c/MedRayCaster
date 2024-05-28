use egui::{Color32, Response, Ui};
use egui_plot::{Bar, BarChart, Legend, Plot};

use crate::{renderer::Scene, shader::ShaderType, volume::Volume};

pub struct UserInterface;

pub struct FrameTimer {
    pub start_time: std::time::Instant,
    pub frame_count: u32,
    pub fps: f64,
}

impl UserInterface {
    pub fn render_histogram(ui: &mut Ui, volume: &Volume) -> Response {
        let bars = volume
            .histogram
            .iter()
            .enumerate()
            .map(|(x, index)| Bar::new(x as f64, *index as f64))
            .collect();
        let chart = BarChart::new(bars).color(Color32::LIGHT_BLUE);

        Plot::new("Histogram")
            .legend(Legend::default())
            .clamp_grid(true)
            .allow_zoom(false)
            .allow_drag(false)
            .allow_scroll(false)
            .height(200.0)
            .show(ui, |plot_ui| plot_ui.bar_chart(chart))
            .response
    }
    pub fn render_controls(ui: &mut Ui, scene: &mut Scene) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 10.0;
            ui.vertical(|ui| {
                ui.add(
                    egui::Slider::new(&mut scene.lower_threshold, 0..=255).text("Lower Threshold"),
                );
                ui.add(
                    egui::Slider::new(&mut scene.upper_threshold, 0..=255).text("Upper Threshold"),
                );
                ui.radio_value(
                    &mut scene.shader_type,
                    ShaderType::DefaultShader,
                    "Default shader",
                );
                ui.radio_value(&mut scene.shader_type, ShaderType::MipShader, "MIP shader");
                ui.radio_value(&mut scene.shader_type, ShaderType::AipShader, "AIP shader");
                ui.label(format!("FPS: {:.2}", scene.frame_timer.fps));
            });
            ui.vertical(|ui| {
                ui.add(
                    egui::Slider::new(&mut scene.camera.camera_x, -2.5..=2.5).text("Translation X"),
                );
                ui.add(
                    egui::Slider::new(&mut scene.camera.camera_y, -2.5..=2.5).text("Translation Y"),
                );
                ui.add(
                    egui::Slider::new(&mut scene.camera.camera_z, -2.5..=2.5).text("Translation Z"),
                );
            });
            ui.vertical(|ui| {
                ui.add(
                    egui::Slider::new(&mut scene.camera.rotation_x, 0.0..=360.0).text("Rotation X"),
                );
                ui.add(
                    egui::Slider::new(&mut scene.camera.rotation_y, 0.0..=360.0).text("Rotation Y"),
                );
                ui.add(
                    egui::Slider::new(&mut scene.camera.rotation_z, 0.0..=360.0).text("Rotation Z"),
                );
            });
        });
    }
}

impl FrameTimer {
    pub fn update_frames_per_second(self: &mut FrameTimer) -> () {
        let now = std::time::Instant::now();
        let elapsed = now - self.start_time;
        if elapsed.as_secs() > 0 {
            let fps = self.frame_count as f64 / elapsed.as_secs_f64();
            self.frame_count = 0;
            self.start_time = now;
            self.fps = fps;
        }
        self.frame_count += 1;
    }
}

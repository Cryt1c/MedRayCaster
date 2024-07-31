use egui_plot::{Bar, BarChart, Legend, Plot};
use three_d::egui::{Color32, Response, Slider, Ui};

use crate::{renderer::Scene, shader::ShaderType, volume::Volume};

pub struct UserInterface;

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
                ui.add(Slider::new(&mut scene.lower_threshold, 0..=255).text("Lower Threshold"));
                ui.add(Slider::new(&mut scene.upper_threshold, 0..=255).text("Upper Threshold"));
                ui.radio_value(
                    &mut scene.shader_type,
                    ShaderType::DefaultShader,
                    "Default shader",
                );
                ui.radio_value(&mut scene.shader_type, ShaderType::MipShader, "MIP shader");
                ui.radio_value(&mut scene.shader_type, ShaderType::AipShader, "AIP shader");
            });
            ui.vertical(|ui| {
                ui.add(Slider::new(&mut scene.camera.location.x, -2.5..=2.5).text("Translation X"));
                ui.add(Slider::new(&mut scene.camera.location.y, -2.5..=2.5).text("Translation Y"));
                ui.add(Slider::new(&mut scene.camera.location.z, -2.5..=2.5).text("Translation Z"));
            });
            ui.vertical(|ui| {
                ui.add(Slider::new(&mut scene.camera.rotation.x, 0.0..=360.0).text("Rotation X"));
                ui.add(Slider::new(&mut scene.camera.rotation.y, 0.0..=360.0).text("Rotation Y"));
                ui.add(Slider::new(&mut scene.camera.rotation.z, 0.0..=360.0).text("Rotation Z"));
            });
        });
    }
}

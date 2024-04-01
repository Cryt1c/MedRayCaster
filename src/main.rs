mod renderer;
mod volume;
mod uniform;
mod shader;

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([350.0, 380.0]),
        multisampling: 4,
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };
    let _ = eframe::run_native(
        "Raycaster",
        options,
        Box::new(|cc| Box::new(renderer::Renderer::new(cc))),
    );
}

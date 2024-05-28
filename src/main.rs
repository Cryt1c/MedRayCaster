mod renderer;
mod shader;
mod ui;
mod uniform;
mod volume;

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([500.0, 500.0]),
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

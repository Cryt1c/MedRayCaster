use egui::{Color32, Response, Ui};
use egui_plot::{Bar, BarChart, Legend, Plot};

use crate::volume::Volume;

pub struct FrameTimer {
    pub start_time: std::time::Instant,
    pub frame_count: u32,
    pub fps: f64,
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

pub struct UserInterface {
    pub lower_threshold: u8,
    pub upper_threshold: u8,
    pub frame_timer: FrameTimer,
}

impl UserInterface {
    pub fn plot_histogram(ui: &mut Ui, volume: &Volume) -> Response {
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
}

pub struct Camera {
    pub camera_x: f32,
    pub camera_y: f32,
    pub camera_z: f32,
    pub rotation_x: f32,
    pub rotation_y: f32,
    pub rotation_z: f32,
}

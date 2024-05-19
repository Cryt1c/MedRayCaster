use crate::shader::Shader;
use crate::volume::Volume;
use egui::{Color32, Response, Ui};
use egui_plot::{Bar, BarChart, Legend, Plot};
use glow::{HasContext, NativeBuffer, NativeTexture, NativeVertexArray};
use nalgebra::{Matrix4, Vector3};
use std::{mem, sync::Arc};

pub struct Renderer {
    start_time: std::time::Instant,
    frame_count: u32,
    fps: f64,
    pub gl_glow: Arc<glow::Context>,
    pub vbo: Option<NativeBuffer>,
    pub vao: Option<NativeVertexArray>,
    pub ebo: Option<NativeBuffer>,
    pub texture: Option<NativeTexture>,
    pub volume: Volume,
    pub mip_shader: bool,
    pub camera_x: f32,
    pub camera_y: f32,
    pub camera_z: f32,
    pub rotation_x: f32,
    pub rotation_y: f32,
    pub rotation_z: f32,
    pub lower_threshold: u8,
    pub upper_threshold: u8,
}

impl Renderer {
    pub fn new(creation_context: &eframe::CreationContext<'_>) -> Self {
        let gl = creation_context
            .gl
            .as_ref()
            .expect("You need to run eframe with the glow backend");

        let mut renderer = Renderer {
            start_time: std::time::Instant::now(),
            frame_count: 0,
            fps: 0.0,
            gl_glow: gl.clone(),
            vao: None,
            vbo: None,
            ebo: None,
            texture: None,
            volume: Volume::new(),
            mip_shader: false,
            camera_x: 0.0,
            camera_y: 0.0,
            camera_z: -2.5,
            rotation_x: 0.0,
            rotation_y: 0.0,
            rotation_z: 0.0,
            lower_threshold: 0,
            upper_threshold: 255,
        };
        renderer.create_vao();
        renderer.create_vbo();
        renderer.create_ebo();
        renderer.create_texture();
        renderer
    }
    pub fn create_vao(&mut self) {
        unsafe {
            self.vao = self.gl_glow.create_vertex_array().ok();
            self.gl_glow.bind_vertex_array(self.vao);
        }
    }
    pub fn create_vbo(&mut self) {
        unsafe {
            self.vbo = self.gl_glow.create_buffer().ok();
            self.gl_glow.bind_buffer(glow::ARRAY_BUFFER, self.vbo);
            self.gl_glow.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&self.volume.vertex_data),
                glow::STATIC_DRAW,
            );
        }
    }
    pub fn create_ebo(&mut self) {
        unsafe {
            self.ebo = self.gl_glow.create_buffer().ok();
            self.gl_glow
                .bind_buffer(glow::ELEMENT_ARRAY_BUFFER, self.ebo);
            self.gl_glow.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                bytemuck::cast_slice(&self.volume.indices),
                glow::STATIC_DRAW,
            );
            self.gl_glow.vertex_attrib_pointer_f32(
                0,
                3,
                glow::FLOAT,
                false,
                3 * mem::size_of::<f32>() as i32,
                0,
            );
            self.gl_glow.enable_vertex_attrib_array(0);
        }
    }
    pub fn create_texture(&mut self) {
        unsafe {
            self.texture = self.gl_glow.create_texture().ok();
            self.gl_glow.bind_texture(glow::TEXTURE_3D, self.texture);
            self.gl_glow.tex_parameter_i32(
                glow::TEXTURE_3D,
                glow::TEXTURE_MIN_FILTER,
                glow::LINEAR as i32,
            );
            self.gl_glow.tex_parameter_i32(
                glow::TEXTURE_3D,
                glow::TEXTURE_MAG_FILTER,
                glow::LINEAR as i32,
            );
            self.gl_glow.tex_parameter_i32(
                glow::TEXTURE_3D,
                glow::TEXTURE_WRAP_S,
                glow::CLAMP_TO_EDGE as i32,
            );
            self.gl_glow.tex_parameter_i32(
                glow::TEXTURE_3D,
                glow::TEXTURE_WRAP_T,
                glow::CLAMP_TO_EDGE as i32,
            );
            self.gl_glow.tex_parameter_i32(
                glow::TEXTURE_3D,
                glow::TEXTURE_WRAP_R,
                glow::CLAMP_TO_EDGE as i32,
            );

            self.gl_glow.tex_image_3d(
                glow::TEXTURE_3D,
                0,
                glow::RGB as i32,
                self.volume.texture.dimensions.width,
                self.volume.texture.dimensions.height,
                self.volume.texture.dimensions.depth,
                0,
                glow::RED,
                glow::UNSIGNED_BYTE,
                Some(bytemuck::cast_slice(&self.volume.texture.texture_data)),
            );
            self.gl_glow.generate_mipmap(glow::TEXTURE_3D);
        }
    }
    fn plot_histogram(&self, ui: &mut Ui) -> Response {
        let bars = self
            .volume
            .histogram
            .iter()
            .enumerate()
            .map(|(x, index)| Bar::new(*index as f64, x as f64))
            .collect();
        let chart = BarChart::new(bars).color(Color32::LIGHT_BLUE);

        Plot::new("Histogram")
            .legend(Legend::default())
            .clamp_grid(true)
            .allow_zoom(false)
            .allow_drag(false)
            .allow_scroll(false)
            .show(ui, |plot_ui| plot_ui.bar_chart(chart))
            .response
    }
    pub fn update_frames_per_second(&mut self) -> () {
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

impl eframe::App for Renderer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_frames_per_second();
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 10.0;
                ui.vertical(|ui| {
                    ui.add(
                        egui::Slider::new(&mut self.lower_threshold, 0..=255)
                            .text("Lower Threshold"),
                    );
                    ui.add(
                        egui::Slider::new(&mut self.upper_threshold, 0..=255)
                            .text("Upper Threshold"),
                    );
                    ui.checkbox(&mut self.mip_shader, "MIP shader");
                    ui.label(format!("FPS: {:.2}", self.fps));
                });
                ui.vertical(|ui| {
                    ui.add(egui::Slider::new(&mut self.camera_x, -2.5..=2.5).text("Translation X"));
                    ui.add(egui::Slider::new(&mut self.camera_y, -2.5..=2.5).text("Translation Y"));
                    ui.add(egui::Slider::new(&mut self.camera_z, -2.5..=2.5).text("Translation Z"));
                });
                ui.vertical(|ui| {
                    ui.add(egui::Slider::new(&mut self.rotation_x, 0.0..=360.0).text("Rotation X"));
                    ui.add(egui::Slider::new(&mut self.rotation_y, 0.0..=360.0).text("Rotation Y"));
                    ui.add(egui::Slider::new(&mut self.rotation_z, 0.0..=360.0).text("Rotation Z"));
                });
                self.plot_histogram(ui);
            });
            if ctx.input(|i| i.zoom_delta() != 1.0) {
                self.camera_z += ctx.input(|i| (i.zoom_delta() - 1.0));
            }
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                let (rect, _) = ui.allocate_exact_size(
                    egui::Vec2::new(ctx.screen_rect().width(), ctx.screen_rect().height()),
                    egui::Sense::drag(),
                );

                // Create local variables to ensure thread safety.
                let texture = self.texture;
                let vao = self.vao;
                let indices_length = self.volume.indices.len();
                let mip_shader = self.mip_shader;
                let camera_x = self.camera_x;
                let camera_y = self.camera_y;
                let camera_z = self.camera_z;
                let rotation_x = self.rotation_x;
                let rotation_y = self.rotation_y;
                let rotation_z = self.rotation_z;
                let screen_rect = ctx.screen_rect();
                let lower_threshold = self.lower_threshold;
                let upper_threshold = self.upper_threshold;

                // Prepare shaders.
                let fragment_shader = if mip_shader {
                    "shaders/mip_shader.glsl"
                } else {
                    "shaders/cookbook_shader.glsl"
                };
                let shaders = Shader::load_from_file("shaders/vertex_shader.glsl", fragment_shader);

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

                            unsafe {
                                painter.gl().bind_texture(glow::TEXTURE_3D, texture);
                                shaders.use_program(painter.gl(), program);

                                let fov_radians = 45.0_f32.to_radians();
                                let aspect_ratio = screen_rect.width() / screen_rect.height();

                                let model_matrix = Matrix4::identity();

                                let mut cam_pos = Vector3::new(camera_x, camera_y, camera_z);
                                let mut up = Vector3::new(0.0, 1.0, 0.0);
                                let origin = Vector3::new(0.0, 0.0, 0.0);

                                cam_pos =
                                    nalgebra_glm::rotate_x_vec3(&cam_pos, rotation_x.to_radians());
                                cam_pos =
                                    nalgebra_glm::rotate_y_vec3(&cam_pos, rotation_y.to_radians());
                                cam_pos =
                                    nalgebra_glm::rotate_z_vec3(&cam_pos, rotation_z.to_radians());

                                up = nalgebra_glm::rotate_x_vec3(&up, rotation_x.to_radians());
                                up = nalgebra_glm::rotate_y_vec3(&up, rotation_y.to_radians());
                                up = nalgebra_glm::rotate_z_vec3(&up, rotation_z.to_radians());

                                let view_matrix = nalgebra_glm::look_at(&cam_pos, &origin, &up);

                                let projection_matrix = nalgebra_glm::perspective(
                                    fov_radians,
                                    aspect_ratio,
                                    0.1,
                                    100.0,
                                );

                                Shader::set_uniform_value(painter.gl(), program, "camPos", cam_pos);
                                Shader::set_uniform_value(painter.gl(), program, "M", model_matrix);
                                Shader::set_uniform_value(painter.gl(), program, "V", view_matrix);
                                Shader::set_uniform_value(
                                    painter.gl(),
                                    program,
                                    "P",
                                    projection_matrix,
                                );
                                Shader::set_uniform_value(
                                    painter.gl(),
                                    program,
                                    "lower_threshold",
                                    lower_threshold,
                                );
                                Shader::set_uniform_value(
                                    painter.gl(),
                                    program,
                                    "upper_threshold",
                                    upper_threshold,
                                );

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
        ctx.request_repaint();
    }
}

use crate::shader::{Shader, ShaderType};
use crate::volume::Volume;
use crate::ui::UserInterface;
use crate::ui::FrameTimer;
use crate::ui::Camera;
use egui::{Style, Visuals};
use glow::{HasContext, NativeBuffer, NativeTexture, NativeVertexArray};
use nalgebra::{Matrix4, Vector3};
use std::{mem, sync::Arc};

pub struct Renderer {
    pub gl_glow: Arc<glow::Context>,
    pub vbo: Option<NativeBuffer>,
    pub vao: Option<NativeVertexArray>,
    pub ebo: Option<NativeBuffer>,
    pub texture: Option<NativeTexture>,
    pub volume: Volume,
    pub camera: Camera,
    pub shader_type: ShaderType,
    pub ui: UserInterface,
}

impl Renderer {
    pub fn new(creation_context: &eframe::CreationContext<'_>) -> Self {
        let gl = creation_context
            .gl
            .as_ref()
            .expect("You need to run eframe with the glow backend");

        let frame_timer = FrameTimer {
            start_time: std::time::Instant::now(),
            frame_count: 0,
            fps: 0.0,
        };

        let camera = Camera {
            camera_x: 0.0,
            camera_y: 0.0,
            camera_z: -2.5,
            rotation_x: 90.0,
            rotation_y: 0.0,
            rotation_z: 180.0,
        };

        let ui = UserInterface {
            frame_timer,
            lower_threshold: 0,
            upper_threshold: 255,
        };

        let mut renderer = Renderer {
            gl_glow: gl.clone(),
            vao: None,
            vbo: None,
            ebo: None,
            texture: None,
            volume: Volume::new(),
            shader_type: ShaderType::DefaultShader,
            camera,
            ui,
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
}

impl eframe::App for Renderer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.ui.frame_timer.update_frames_per_second();
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 10.0;
                    ui.vertical(|ui| {
                        ui.add(
                            egui::Slider::new(&mut self.ui.lower_threshold, 0..=255)
                                .text("Lower Threshold"),
                        );
                        ui.add(
                            egui::Slider::new(&mut self.ui.upper_threshold, 0..=255)
                                .text("Upper Threshold"),
                        );
                        ui.radio_value(
                            &mut self.shader_type,
                            ShaderType::DefaultShader,
                            "Default shader",
                        );
                        ui.radio_value(&mut self.shader_type, ShaderType::MipShader, "MIP shader");
                        ui.radio_value(&mut self.shader_type, ShaderType::AipShader, "AIP shader");
                        ui.label(format!("FPS: {:.2}", self.ui.frame_timer.fps));
                    });
                    ui.vertical(|ui| {
                        ui.add(
                            egui::Slider::new(&mut self.camera.camera_x, -2.5..=2.5)
                                .text("Translation X"),
                        );
                        ui.add(
                            egui::Slider::new(&mut self.camera.camera_y, -2.5..=2.5)
                                .text("Translation Y"),
                        );
                        ui.add(
                            egui::Slider::new(&mut self.camera.camera_z, -2.5..=2.5)
                                .text("Translation Z"),
                        );
                    });
                    ui.vertical(|ui| {
                        ui.add(
                            egui::Slider::new(&mut self.camera.rotation_x, 0.0..=360.0)
                                .text("Rotation X"),
                        );
                        ui.add(
                            egui::Slider::new(&mut self.camera.rotation_y, 0.0..=360.0)
                                .text("Rotation Y"),
                        );
                        ui.add(
                            egui::Slider::new(&mut self.camera.rotation_z, 0.0..=360.0)
                                .text("Rotation Z"),
                        );
                    });
                });
                UserInterface::plot_histogram(ui, &self.volume);
            });
            if ctx.input(|i| i.zoom_delta() != 1.0) {
                self.camera.camera_z += ctx.input(|i| (i.zoom_delta() - 1.0));
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

                // Create local variables to ensure thread safety.
                let texture = self.texture;
                let vao = self.vao;
                let indices_length = self.volume.indices.len();
                let camera = &self.camera;
                let camera_x = camera.camera_x;
                let camera_y = camera.camera_y;
                let camera_z = camera.camera_z;
                let rotation_x = camera.rotation_x;
                let rotation_y = camera.rotation_y;
                let rotation_z = camera.rotation_z;
                let lower_threshold = self.ui.lower_threshold;
                let upper_threshold = self.ui.upper_threshold;

                let fragment_shader = match self.shader_type {
                    ShaderType::DefaultShader => "shaders/cookbook_shader.glsl",
                    ShaderType::MipShader => "shaders/mip_shader.glsl",
                    ShaderType::AipShader => "shaders/aip_shader.glsl",
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

                                let aspect_ratio = rect.aspect_ratio();

                                let view_matrix = nalgebra_glm::look_at(&cam_pos, &origin, &up);

                                let projection_matrix = nalgebra_glm::perspective(
                                    fov_radians,
                                    aspect_ratio,
                                    0.1,
                                    100.0,
                                );

                                Shader::set_uniform_value(
                                    painter.gl(),
                                    program,
                                    "cam_pos",
                                    cam_pos,
                                );
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

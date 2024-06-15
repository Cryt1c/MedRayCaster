use crate::shader::{Shader, ShaderType};
use crate::ui::FrameTimer;
use crate::ui::UserInterface;
use crate::volume::Volume;
use egui::{Style, Visuals};
use glow::{HasContext, NativeBuffer, NativeTexture, NativeVertexArray};
use nalgebra::{Matrix4, Vector3};
use std::{mem, sync::Arc};
use three_d::Context;

pub struct Renderer {
    pub gl: Arc<three_d::Context>,
    pub vbo: Option<NativeBuffer>,
    pub vao: Option<NativeVertexArray>,
    pub ebo: Option<NativeBuffer>,
    pub texture: Option<NativeTexture>,
    pub scene: Scene,
}

pub struct Scene {
    pub volume: Volume,
    pub camera: Camera,
    pub shader_type: ShaderType,
    pub lower_threshold: u8,
    pub upper_threshold: u8,
    pub frame_timer: FrameTimer,
}

pub struct Camera {
    pub aspect_ratio: f32,
    pub location: Vector3<f32>,
    pub rotation: Vector3<f32>,
}

pub struct Uniforms {
    pub cam_pos: Vector3<f32>,
    pub model_matrix: Matrix4<f32>,
    pub view_matrix: Matrix4<f32>,
    pub projection_matrix: Matrix4<f32>,
    pub lower_threshold: u8,
    pub upper_threshold: u8,
}

impl Renderer {
    pub fn new(context: Context) -> Self {
        let arc_context = Arc::new(context.clone());

        let frame_timer = FrameTimer {
            start_time: std::time::Instant::now(),
            frame_count: 0,
            fps: 0.0,
        };

        let camera = Camera {
            aspect_ratio: 1.0,
            location: Vector3::new(0.0, 0.0, -2.5),
            rotation: Vector3::new(90.0, 0.0, 180.0),
        };

        let mut renderer = Renderer {
            gl: arc_context,
            vao: None,
            vbo: None,
            ebo: None,
            texture: None,
            scene: Scene {
                volume: Volume::new(),
                camera,
                shader_type: ShaderType::DefaultShader,
                frame_timer,
                lower_threshold: 0,
                upper_threshold: 255,
            },
        };
        renderer.create_vao();
        renderer.create_vbo();
        renderer.create_ebo();
        renderer.create_texture();
        renderer
    }
    pub fn create_vao(&mut self) {
        unsafe {
            self.vao = self.gl.create_vertex_array().ok();
            self.gl.bind_vertex_array(self.vao);
        }
    }
    pub fn create_vbo(&mut self) {
        unsafe {
            self.vbo = self.gl.create_buffer().ok();
            self.gl.bind_buffer(glow::ARRAY_BUFFER, self.vbo);
            self.gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&self.scene.volume.vertex_data),
                glow::STATIC_DRAW,
            );
        }
    }
    pub fn create_ebo(&mut self) {
        unsafe {
            self.ebo = self.gl.create_buffer().ok();
            self.gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, self.ebo);
            self.gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                bytemuck::cast_slice(&self.scene.volume.indices),
                glow::STATIC_DRAW,
            );
            self.gl.vertex_attrib_pointer_f32(
                0,
                3,
                glow::FLOAT,
                false,
                3 * mem::size_of::<f32>() as i32,
                0,
            );
            self.gl.enable_vertex_attrib_array(0);
        }
    }
    pub fn create_texture(&mut self) {
        unsafe {
            self.texture = self.gl.create_texture().ok();
            self.gl.bind_texture(glow::TEXTURE_3D, self.texture);
            self.gl.tex_parameter_i32(
                glow::TEXTURE_3D,
                glow::TEXTURE_MIN_FILTER,
                glow::LINEAR as i32,
            );
            self.gl.tex_parameter_i32(
                glow::TEXTURE_3D,
                glow::TEXTURE_MAG_FILTER,
                glow::LINEAR as i32,
            );
            self.gl.tex_parameter_i32(
                glow::TEXTURE_3D,
                glow::TEXTURE_WRAP_S,
                glow::CLAMP_TO_EDGE as i32,
            );
            self.gl.tex_parameter_i32(
                glow::TEXTURE_3D,
                glow::TEXTURE_WRAP_T,
                glow::CLAMP_TO_EDGE as i32,
            );
            self.gl.tex_parameter_i32(
                glow::TEXTURE_3D,
                glow::TEXTURE_WRAP_R,
                glow::CLAMP_TO_EDGE as i32,
            );

            self.gl.tex_image_3d(
                glow::TEXTURE_3D,
                0,
                glow::RGB as i32,
                self.scene.volume.texture.dimensions.width,
                self.scene.volume.texture.dimensions.height,
                self.scene.volume.texture.dimensions.depth,
                0,
                glow::RED,
                glow::UNSIGNED_BYTE,
                Some(bytemuck::cast_slice(
                    &self.scene.volume.texture.texture_data,
                )),
            );
            self.gl.generate_mipmap(glow::TEXTURE_3D);
        }
    }

    fn calculate_model_matrix(&self) -> Matrix4<f32> {
        Matrix4::identity()
    }

    fn calculate_view_matrix(&self, cam_pos: Vector3<f32>) -> Matrix4<f32> {
        let mut up = Vector3::new(0.0, 1.0, 0.0);
        up = nalgebra_glm::rotate_x_vec3(&up, self.scene.camera.rotation.x.to_radians());
        up = nalgebra_glm::rotate_y_vec3(&up, self.scene.camera.rotation.y.to_radians());
        up = nalgebra_glm::rotate_z_vec3(&up, self.scene.camera.rotation.z.to_radians());
        let origin = Vector3::new(0.0, 0.0, 0.0);
        nalgebra_glm::look_at(&cam_pos, &origin, &up)
    }

    fn calculate_projection_matrix(&self) -> Matrix4<f32> {
        let fov_radians = 45.0_f32.to_radians();
        let aspect_ratio = self.scene.camera.aspect_ratio;
        nalgebra_glm::perspective(fov_radians, aspect_ratio, 0.1, 100.0)
    }

    pub fn calculate_uniforms(&self) -> Uniforms {
        let cam_pos = self.scene.camera.calculate_cam_pos();
        Uniforms {
            cam_pos,
            model_matrix: self.calculate_model_matrix(),
            view_matrix: self.calculate_view_matrix(cam_pos),
            projection_matrix: self.calculate_projection_matrix(),
            lower_threshold: self.scene.lower_threshold,
            upper_threshold: self.scene.upper_threshold,
        }
    }

    pub fn set_uniform_values(
        uniforms: &Uniforms,
        painter: &egui_glow::Painter,
        program: glow::NativeProgram,
    ) {
        Shader::set_uniform_value(painter.gl(), program, "cam_pos", uniforms.cam_pos);
        Shader::set_uniform_value(painter.gl(), program, "M", uniforms.model_matrix);
        Shader::set_uniform_value(painter.gl(), program, "V", uniforms.view_matrix);
        Shader::set_uniform_value(painter.gl(), program, "P", uniforms.projection_matrix);
        Shader::set_uniform_value(
            painter.gl(),
            program,
            "lower_threshold",
            uniforms.lower_threshold,
        );
        Shader::set_uniform_value(
            painter.gl(),
            program,
            "upper_threshold",
            uniforms.upper_threshold,
        );
    }
}

impl Camera {
    pub fn calculate_cam_pos(&self) -> Vector3<f32> {
        let mut cam_pos = self.location;
        cam_pos = nalgebra_glm::rotate_x_vec3(&cam_pos, self.rotation.x.to_radians());
        cam_pos = nalgebra_glm::rotate_y_vec3(&cam_pos, self.rotation.y.to_radians());
        cam_pos = nalgebra_glm::rotate_z_vec3(&cam_pos, self.rotation.z.to_radians());
        cam_pos
    }
}

impl eframe::App for Renderer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.scene.frame_timer.update_frames_per_second();
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                UserInterface::render_controls(ui, &mut self.scene);
                UserInterface::render_histogram(ui, &self.scene.volume);
            });
            if ctx.input(|i| i.zoom_delta() != 1.0) {
                self.scene.camera.location.z += ctx.input(|i| (i.zoom_delta() - 1.0));
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
                self.scene.camera.aspect_ratio = rect.aspect_ratio();

                // Create local variables to ensure thread safety.
                let texture = self.texture;
                let vao = self.vao;
                let indices_length = self.scene.volume.indices.len();
                let uniforms = self.calculate_uniforms();

                let fragment_shader = match self.scene.shader_type {
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
    }
}

#[cfg(test)]
mod test {
    use crate::renderer::Camera;
    use approx::assert_relative_eq;
    use nalgebra::Vector3;
    const EPSILON: f32 = 0.001;

    #[test]
    fn test_camera_no_rotation() {
        let camera = Camera {
            aspect_ratio: 1.0,
            location: Vector3::new(0.0, 0.0, -2.5),
            rotation: Vector3::new(0.0, 0.0, 0.0),
        };
        let cam_pos = camera.calculate_cam_pos();
        assert_eq!(cam_pos.x, 0.0);
        assert_eq!(cam_pos.y, 0.0);
        assert_eq!(cam_pos.z, -2.5);
    }

    #[test]
    fn test_camera_rotation_x() {
        let camera = Camera {
            aspect_ratio: 1.0,
            location: Vector3::new(0.0, 0.0, -1.0),
            rotation: Vector3::new(90.0, 0.0, 0.0),
        };
        let cam_pos = camera.calculate_cam_pos().normalize();
        assert_relative_eq!(cam_pos.x, 0.0, epsilon = EPSILON);
        assert_relative_eq!(cam_pos.y, 1.0, epsilon = EPSILON);
        assert_relative_eq!(cam_pos.z, 0.0, epsilon = EPSILON);
    }

    #[test]
    fn test_camera_rotation_y() {
        let camera = Camera {
            aspect_ratio: 1.0,
            location: Vector3::new(0.0, 0.0, -1.0),
            rotation: Vector3::new(0.0, 90.0, 0.0),
        };
        let cam_pos = camera.calculate_cam_pos().normalize();
        assert_relative_eq!(cam_pos.x, -1.0, epsilon = EPSILON);
        assert_relative_eq!(cam_pos.y, 0.0, epsilon = EPSILON);
        assert_relative_eq!(cam_pos.z, 0.0, epsilon = EPSILON);
    }

    #[test]
    fn test_camera_rotation_z() {
        let camera = Camera {
            aspect_ratio: 1.0,
            location: Vector3::new(0.0, 0.0, -1.0),
            rotation: Vector3::new(0.0, 0.0, 90.0),
        };
        let cam_pos = camera.calculate_cam_pos().normalize();
        assert_relative_eq!(cam_pos.x, 0.0, epsilon = EPSILON);
        assert_relative_eq!(cam_pos.y, 0.0, epsilon = EPSILON);
        assert_relative_eq!(cam_pos.z, -1.0, epsilon = EPSILON);
    }
}

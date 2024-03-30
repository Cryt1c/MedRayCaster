use glow::{HasContext, NativeBuffer, NativeTexture, NativeVertexArray};
use opengl_rs::shader::Shader;
use std::{mem, sync::Arc};

use crate::volume::Volume;

pub struct Renderer {
    start_time: std::time::Instant,
    frame_count: u32,
    pub gl_glow: Arc<glow::Context>,
    pub vbo: Option<NativeBuffer>,
    pub vao: Option<NativeVertexArray>,
    pub ebo: Option<NativeBuffer>,
    pub texture: Option<NativeTexture>,
    pub volume: Volume,
}

impl Renderer {
    pub fn new(creation_context: &eframe::CreationContext<'_>) -> Self {
        let gl = creation_context
            .gl
            .as_ref()
            .expect("You need to run eframe with the glow backend");

        let mut renderer = Renderer {
            gl_glow: gl.clone(),
            vao: None,
            vbo: None,
            ebo: None,
            texture: None,
            frame_count: 0,
            start_time: std::time::Instant::now(),
            volume: Volume::new(),
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
                self.volume.width as i32,
                self.volume.height as i32,
                self.volume.depth as i32,
                0,
                glow::RED,
                glow::UNSIGNED_BYTE,
                Some(bytemuck::cast_slice(&self.volume.texture_data)),
            );
            self.gl_glow.generate_mipmap(glow::TEXTURE_3D);
        }
    }
}

impl eframe::App for Renderer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let shaders =
            Shader::load_from_file("shaders/vertex_shader.glsl", "shaders/raycaster.glsl");
        // Create GLSL shaders
        let vs = shaders.compile_shader(&self.gl_glow, shaders.get_vertex(), glow::VERTEX_SHADER);
        let fs =
            shaders.compile_shader(&self.gl_glow, shaders.get_fragment(), glow::FRAGMENT_SHADER);
        let program = shaders.link_program(&self.gl_glow, vs, fs);

        let screen_rect = ctx.available_rect();
        unsafe {
            // Clear the screen to black
            self.gl_glow.clear_color(0.0, 0.0, 0.0, 1.0);
            self.gl_glow.clear(glow::COLOR_BUFFER_BIT);
            self.gl_glow.bind_texture(glow::TEXTURE_3D, self.texture);

            // Use shader program
            shaders.use_program(&self.gl_glow, program);

            shaders.set_uniform_values(&self.gl_glow, program, screen_rect);

            self.gl_glow.bind_vertex_array(self.vao);
            self.gl_glow.draw_elements(
                glow::TRIANGLES,
                self.volume.indices.len() as i32,
                glow::UNSIGNED_INT,
                0,
            );
            if self.gl_glow.get_error() != glow::NO_ERROR {
                println!("Error: {}", self.gl_glow.get_error());
            }
        }
        ctx.request_repaint();
        // let _ = event_loop.run(move |event, elwt| {
        //     match event {
        //         Event::LoopExiting => return,
        //         Event::WindowEvent { event, .. } => match event {
        //             WindowEvent::CloseRequested => {
        //                 // Cleanup
        //                 unsafe {
        //                     shaders.delete_program(&self.gl, program);
        //                     shaders.delete_shader(&self.gl, fs);
        //                     shaders.delete_shader(&self.gl, vs);
        //                     self.gl.delete_vertex_array(self.vao.unwrap());
        //                     self.gl.delete_buffer(self.vbo.unwrap());
        //                     self.gl.delete_buffer(self.ebo.unwrap());
        //                 }
        //                 elwt.exit();
        //             }
        //             WindowEvent::RedrawRequested => {
        //                 // gl_surface
        //                 //     .swap_buffers(&gl_possibly_current_context)
        //                 //     .unwrap();
        //             }
        //             _ => (),
        //         },
        //         _ => (),
        //     }
        //     // gl_window.request_redraw();
        // });
        // egui::CentralPanel::default().show(ctx, |ui| {
        //     ui.horizontal(|ui| {
        //         ui.spacing_mut().item_spacing.x = 0.0;
        //         ui.label("The triangle is being painted using ");
        //         ui.hyperlink_to("glow", "https://github.com/grovesNL/glow");
        //         ui.label(" (OpenGL).");
        //     });
        //
        //     egui::Frame::canvas(ui.style()).show(ui, |ui| {
        //         // self.custom_painting(ui);
        //     });
        //     ui.label("Drag to rotate!");
        // });
        self.frame_count += 1;
        let now = std::time::Instant::now();
        let elapsed = now - self.start_time;
        if elapsed.as_secs() > 0 {
            let fps = self.frame_count as f64 / elapsed.as_secs_f64();
            println!("FPS: {}", fps);
            self.frame_count = 0;
            self.start_time = now;
        }
    }
}

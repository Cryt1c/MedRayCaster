use glow::{HasContext, NativeBuffer, NativeTexture, NativeVertexArray};
use glutin::{
    config::ConfigTemplateBuilder,
    context::{ContextAttributesBuilder, NotCurrentGlContext},
    display::{GetGlDisplay, GlDisplay},
    surface::{GlSurface, SwapInterval, WindowSurface},
};
use glutin_winit::{DisplayBuilder, GlWindow};
use std::mem;
use std::num::NonZeroU32;

use crate::volume::Volume;

pub struct Renderer {
    pub gl_glow: glow::Context,
    pub gl_surface: glutin::surface::Surface<WindowSurface>,
    pub gl_window: winit::window::Window,
    pub gl_possibly_current_context: glutin::context::PossiblyCurrentContext,
    pub event_loop: winit::event_loop::EventLoop<()>,
    pub vbo: Option<NativeBuffer>,
    pub vao: Option<NativeVertexArray>,
    pub ebo: Option<NativeBuffer>,
    pub texture: Option<NativeTexture>,
}

impl Renderer {
    pub fn new() -> Self {
        let event_loop = winit::event_loop::EventLoopBuilder::new().build().unwrap();
        let window_builder = winit::window::WindowBuilder::new()
            .with_title("Raycaster")
            .with_inner_size(winit::dpi::LogicalSize::new(1024.0, 768.0));
        let template = ConfigTemplateBuilder::new();
        let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));
        let (maybe_window, gl_config) = display_builder
            .build(&event_loop, template, |mut configs| configs.nth(0).unwrap())
            .unwrap();
        let context_attributes = ContextAttributesBuilder::new().build(None);
        let gl_window = maybe_window.unwrap();
        let attrs = gl_window.build_surface_attributes(Default::default());

        unsafe {
            let gl_surface = gl_config
                .display()
                .create_window_surface(&gl_config, &attrs)
                .unwrap();
            let gl_display = gl_config.display();
            let gl_possibly_current_context = gl_display
                .create_context(&gl_config, &context_attributes)
                .unwrap()
                .make_current(&gl_surface)
                .unwrap();
            let gl_glow =
                glow::Context::from_loader_function_cstr(|s| gl_display.get_proc_address(s));
            gl_surface
                .set_swap_interval(
                    &gl_possibly_current_context,
                    SwapInterval::Wait(NonZeroU32::new(1).unwrap()),
                )
                .unwrap();
            Renderer {
                gl_glow,
                gl_surface,
                gl_window,
                gl_possibly_current_context,
                event_loop,
                vao: None,
                vbo: None,
                ebo: None,
                texture: None,
            }
        }
    }
    pub fn create_vao(&mut self) {
        unsafe {
            self.vao = self.gl_glow.create_vertex_array().ok();
            self.gl_glow.bind_vertex_array(self.vao);
        }
    }
    pub fn create_vbo(&mut self, volume: &Volume) {
        unsafe {
            self.vbo = self.gl_glow.create_buffer().ok();
            self.gl_glow.bind_buffer(glow::ARRAY_BUFFER, self.vbo);
            self.gl_glow.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&volume.vertex_data),
                glow::STATIC_DRAW,
            );
        }
    }
    pub fn create_ebo(&mut self, volume: &Volume) {
        unsafe {
            self.ebo = self.gl_glow.create_buffer().ok();
            self.gl_glow
                .bind_buffer(glow::ELEMENT_ARRAY_BUFFER, self.ebo);
            self.gl_glow.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                bytemuck::cast_slice(&volume.indices),
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
    pub fn create_texture(&mut self, volume: &Volume) {
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
                volume.width as i32,
                volume.height as i32,
                volume.depth as i32,
                0,
                glow::RED,
                glow::UNSIGNED_BYTE,
                Some(bytemuck::cast_slice(&volume.texture_data)),
            );
            self.gl_glow.generate_mipmap(glow::TEXTURE_3D);
        }
    }
    fn render(&self) {
        println!("Rendering...");
    }
}

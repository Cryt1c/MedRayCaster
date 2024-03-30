use glutin::{
    config::ConfigTemplateBuilder,
    context::{ContextAttributesBuilder, NotCurrentGlContext},
    display::{GetGlDisplay, GlDisplay},
    surface::{GlSurface, SwapInterval, WindowSurface},
};
use glutin_winit::{DisplayBuilder, GlWindow};
use std::num::NonZeroU32;

pub struct Renderer {
    pub gl_glow: glow::Context,
    pub gl_surface: glutin::surface::Surface<WindowSurface>,
    pub gl_window: winit::window::Window,
    pub gl_possibly_current_context: glutin::context::PossiblyCurrentContext,
    pub event_loop: winit::event_loop::EventLoop<()>,
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
                .set_swap_interval(&gl_possibly_current_context, SwapInterval::Wait(NonZeroU32::new(1).unwrap()))
                .unwrap();
            Renderer {
                gl_glow,
                gl_surface,
                gl_window,
                gl_possibly_current_context,
                event_loop,
            }
        }
    }
    fn render(&self) {
        println!("Rendering...");
    }
}

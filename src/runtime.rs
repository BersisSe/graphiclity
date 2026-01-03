use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes, WindowId};

use crate::Config;
use crate::backends::PixelsBackend;
use crate::context::WindowContext;
use crate::graphics::Graphics;
use crate::input::Input;

/// The main runtime struct that manages the application lifecycle
/// It holds the configuration, window, graphics context, rendering backend, and the user-defined drawing function.
pub struct Runtime<F> {
    config: Config,
    window: Option<Window>,
    context: Option<WindowContext>,
    backend: Option<PixelsBackend>,
    draw_fn: F,
}

impl<F> Runtime<F>
where
    F: FnMut(&mut WindowContext),
{
    pub fn new(draw_fn: F, config: Config) -> Self {
        let logical_size = LogicalSize::new(config.logical_width, config.logical_height);

        let graphics = Graphics::new(
            logical_size,
            PhysicalSize::new(config.window_width, config.window_height),
        );

        let inputs = Input::new();

        Self {
            config,
            window: None,
            context: Some(WindowContext::new(graphics, inputs)),
            backend: None,
            draw_fn: draw_fn,
        }
    }
}

impl<F> ApplicationHandler for Runtime<F>
where
    F: FnMut(&mut WindowContext),
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let config = &self.config;

        let attrs = WindowAttributes::default()
            .with_title(&config.title)
            .with_resizable(config.resizeable)
            .with_inner_size(PhysicalSize::new(config.window_width, config.window_height));

        let window = event_loop.create_window(attrs).unwrap();

        let physical_size = window.inner_size();
        let logical_size = LogicalSize::new(config.logical_width, config.logical_height);

        self.backend = Some(PixelsBackend::new(&window, physical_size, logical_size));
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        self.context
            .as_mut()
            .unwrap()
            .inputs
            .helper
            .process_window_event(&event);
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let mut context = self.context.as_mut().unwrap();
                let renderer = self.backend.as_mut().unwrap();
                context.gfx.begin_frame();

                (self.draw_fn)(&mut context);

                renderer.render(context.gfx.commands());
            }
            WindowEvent::Resized(physical_size) => {
                if physical_size.width == 0 || physical_size.height == 0 {
                    return;
                }
                if let Some(renderer) = &mut self.backend {
                    renderer.resize_window(physical_size);
                }
            }

            _ => (),
        }
    }

    fn new_events(&mut self, _: &ActiveEventLoop, _: winit::event::StartCause) {
        self.context.as_mut().unwrap().inputs.helper.step();
    }
    fn device_event(
        &mut self,
        _: &ActiveEventLoop,
        _: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        self.context
            .as_mut()
            .unwrap()
            .inputs
            .helper
            .process_device_event(&event);
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.context.as_mut().unwrap().inputs.helper.end_step();
        if let Some(win) = &self.window {
            win.request_redraw();
        }
    }
}

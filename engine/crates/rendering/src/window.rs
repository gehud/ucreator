use std::time::Duration;

use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::platform::pump_events::{EventLoopExtPumpEvents, PumpStatus};
use winit::window::{Window as Handle, WindowAttributes};

pub struct Window {
    event_loop: EventLoop<()>,
    handler: EventHandler,
    width: u32,
    height: u32,
    exited: bool
}

pub trait Resolution {
    fn width(&self) -> u32;

    fn height(&self) -> u32;
}

impl Resolution for (u32, u32) {
    fn width(&self) -> u32 {
        self.0
    }

    fn height(&self) -> u32 {
        self.1
    }
}

pub struct EventHandler {
    attributes: WindowAttributes,
    handle: Option<winit::window::Window>
}

impl winit::application::ApplicationHandler for EventHandler {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.handle = Some(event_loop.create_window(self.attributes.clone()).unwrap());
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            _ => {}
        }
    }
}

impl Window {
    pub fn new(title: &str, resolution: impl Resolution) -> Self {
        let attributes = Handle::default_attributes()
            .with_title(title)
            .with_inner_size(LogicalSize::new(resolution.width(), resolution.height()))
            .with_visible(true);

        Self {
            event_loop: EventLoop::new().unwrap(),
            handler: EventHandler {
                attributes,
                handle: None
            },
            width: resolution.width(),
            height: resolution.height(),
            exited: false
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

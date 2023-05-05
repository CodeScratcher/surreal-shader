use glium;
use glium::{Surface};
use glium::glutin;
use glium::glutin::{event_loop, dpi, window};

use crate::event_handler::{handle_raw_event, GameEvent};
use crate::model::Model;
use crate::render::Renderer;

pub struct Window {
    pub height: u32,
    pub width: u32,
    event_loop: event_loop::EventLoop<()>,
    pub display: glium::Display,
}

impl Window {
    pub fn default(height: u32, width: u32) -> Self {
        let mut event_loop = event_loop::EventLoop::new();
        let wb = window::WindowBuilder::new()
            .with_inner_size(dpi::PhysicalSize::new(width, height));
        let cb = glutin::ContextBuilder::new();
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();

        Self {
            height,
            width,
            event_loop,
            display
        }
    }

    pub fn run(self, renderer: Renderer, mut model: Model) {
        self.event_loop.run(move |ev, _, control_flow| {

            let next_frame_time = std::time::Instant::now() +
                std::time::Duration::from_nanos(16_666_667);
            *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
            
            let maybe_event = handle_raw_event(ev);

            if let Some(event) = maybe_event {
                match event {
                    GameEvent::Close => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        return;
                    }
                    _ => ()
                }
            }

            model.tick();
            
            renderer.draw(&self.display, &model);
        });
    }
}
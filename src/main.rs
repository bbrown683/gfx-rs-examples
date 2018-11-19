#![feature(range_contains)]
extern crate ash;
#[macro_use] extern crate log;
extern crate log4rs;
extern crate winit;

/// High-level wrapper for [ash](https://github.com/MaikKlein/ash) around typical types.
pub mod rhi;
pub mod util;

use log::{LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Logger, Root};
use winit::WindowEvent;
use crate::util::CapturedEvent;

fn main() {
    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout_appender", Box::new(stdout)))
        .build(Root::builder().appender("stdout_appender").build(LevelFilter::Debug))
        .unwrap();

    let handle = log4rs::init_config(config).unwrap();


    let mut events_loop = winit::EventsLoop::new();
    let window = winit::WindowBuilder::new()
        .with_dimensions(winit::dpi::LogicalSize::new(1024 as _, 768 as _))
        .with_title("Halogen".to_string())
        .build(&events_loop)
        .expect("Failed to create window.");

    let mut renderer = rhi::Renderer::new(&window);

    let mut running = true;
    &renderer.begin_frame();
    &renderer.end_frame();
    while running {
        events_loop.poll_events(|event| {
            if let winit::Event::WindowEvent { event, .. } = event {
                match event {
                    WindowEvent::CloseRequested => running = false,
                    WindowEvent::CursorMoved {
                            device_id, position, modifiers
                    } => (),
                    WindowEvent::KeyboardInput {
                        device_id, input
                    } => renderer.on_keyboard_input(input),
                    WindowEvent::MouseInput {
                        device_id, state, button, modifiers
                    } => renderer.on_mouse_input(button),
                    WindowEvent::Resized(size) => renderer.on_resize(size),
                    _ => (),
                }
            }
        });
    }
}
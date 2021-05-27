#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused))]

#[global_allocator]
static ALLOC: rpmalloc::RpMalloc = rpmalloc::RpMalloc;

use maligog::vk;
use tracing::Level;
use winit::event_loop::ControlFlow;

mod engine;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();
    tracing::debug!("fuck");
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_inner_size(winit::dpi::PhysicalSize::new(800, 600))
        .build(&event_loop)
        .unwrap();
    let engine = engine::Engine::new();
    event_loop.run(|event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            winit::event::Event::NewEvents(_) => {}
            winit::event::Event::WindowEvent { window_id, event } => {
                match event {
                    winit::event::WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    winit::event::WindowEvent::KeyboardInput { input, .. } => {
                        if let Some(key) = input.virtual_keycode {
                            match key {
                                winit::event::VirtualKeyCode::Escape => {
                                    *control_flow = ControlFlow::Exit;
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
            winit::event::Event::DeviceEvent { device_id, event } => {}
            winit::event::Event::UserEvent(_) => {}
            winit::event::Event::Suspended => {}
            winit::event::Event::Resumed => {}
            winit::event::Event::MainEventsCleared => {}
            winit::event::Event::RedrawRequested(_) => {}
            winit::event::Event::RedrawEventsCleared => {}
            winit::event::Event::LoopDestroyed => {}
        }
    });
}

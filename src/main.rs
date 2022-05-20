extern crate glium;
mod engine;
mod loading;
mod utils;

use std::env;

fn main() {
    use glium::glutin;
    use engine::structures::{Vertex, Normal, Mesh};

    let el = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new().with_title("3D Engine");
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &el).unwrap();

    let mesh = loading::load_obj(String::from(env::args().skip(1).next().unwrap()));

    let mut engine = engine::Engine::new(mesh);

    el.run(move |ev, _, control_flow| {
        engine.draw(&display);

        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            _ => (),
        }
    });
}

extern crate glium;
use glium::glutin;
use crate::engine;
use crate::loading;

use std::env;

pub struct Window {
  engine: engine::Engine,
  event_loop: glutin::event_loop::EventLoop<()>,
  display: Option<glium::Display>
}

impl Window {
  pub fn new() -> Window {
    Window {
      engine: engine::Engine::new(),
      event_loop: glutin::event_loop::EventLoop::new(),
      display: None
    }
  }

  pub fn init(&mut self) {
    let wb = glutin::window::WindowBuilder::new().with_title("3D Engine");
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    self.display = Some(glium::Display::new(wb, cb, &self.event_loop).unwrap());

    let scene = loading::load_scene(String::from(env::args().skip(1).next().unwrap()));
    self.engine.set_scene(scene);    
  }

  pub fn run(mut self) {
    if self.display.is_none() {
      panic!("Display is not available. Please initialize the Window.");
    }

    self.event_loop.run(move |ev, _, control_flow| {
      let mut pressed = Vec::new();
      let mut released = Vec::new();

      let next_frame_time =
          std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
      *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
      match ev {
          glutin::event::Event::WindowEvent { event, .. } => match event {
              glutin::event::WindowEvent::CloseRequested => {
                  *control_flow = glutin::event_loop::ControlFlow::Exit;
                  return;
              }
              glutin::event::WindowEvent::KeyboardInput { device_id, input: kin, is_synthetic: _ } => {
                  if kin.state == glutin::event::ElementState::Pressed {
                      pressed.push(format!("{:?}", kin.virtual_keycode.unwrap()));
                  } else if kin.state == glutin::event::ElementState::Released {
                      released.push(format!("{:?}", kin.virtual_keycode.unwrap()));
                  }
              }
              _ => return,
          },
          _ => (),
      }

      self.engine.draw(&self.display.as_ref().unwrap(), &pressed, &released);
    });
  }
}
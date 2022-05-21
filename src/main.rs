extern crate glium;
mod engine;
mod loading;
mod utils;
mod window;

fn main() {
    let mut window = window::Window::new();
    window.init();
    window.run();
}

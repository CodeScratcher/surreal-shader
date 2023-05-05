use model::Model;
use render::Renderer;

#[macro_use]
extern crate glium;

mod window;
mod model;
mod event_handler;
mod render;
mod teapot;

fn main() {
    let window = crate::window::Window::default(720, 1080);

    let renderer = Renderer::default(&window.display);
    let mut model: Model = Model::default();
    window.run(renderer, model);
}

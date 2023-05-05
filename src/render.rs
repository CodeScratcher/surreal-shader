use std::fs;

use glium::{Display, Surface, Program};

use crate::model::Model;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
}

implement_vertex!(Vertex, position);

pub struct Renderer {
    program: Program,
}

impl Renderer {
    pub fn default(display: &Display) -> Self {
        let vertex_shader_src = fs::read_to_string("shader/shader.vert").unwrap();
        let fragment_shader_src = fs::read_to_string("shader/shader.frag").unwrap();

        let program = glium::Program::from_source(display, &vertex_shader_src, &fragment_shader_src, None).unwrap();
        
        Self {
            program
        }
    }

    pub fn draw(&self, display: &Display, model: &Model) {
        let vertex1 = Vertex { position: [ -0.5, -0.5,  0.0 ] };
        let vertex2 = Vertex { position: [ 0.0,  0.5,   0.0 ] };
        let vertex3 = Vertex { position: [ 0.5,  -0.25, 0.0 ] };
        let shape = vec![vertex1, vertex2, vertex3];
    
        let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        target.draw(&vertex_buffer, &indices, &self.program, &uniform! {matrix: [
            [(model.frame as f32 * 0.0002).cos(), (model.frame as f32 * 0.0002).sin(), 0.0, 0.0],
            [-(model.frame as f32 * 0.0002).sin(), (model.frame as f32 * 0.0002).cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ]},
            &Default::default()).unwrap();
        target.finish().unwrap();
    }
}


pub mod structures;
mod camera;
mod maths;

use glium::{Display, Program, Surface};
use std::fs;
use structures::Mesh;

pub struct Engine {
  camera: camera::Camera,
  mesh: Mesh
}

impl Engine {
    pub fn new(mesh: Mesh) -> Engine {
      Engine {
        camera: camera::Camera::new(),
        mesh: mesh
      }
    }

    pub fn draw(&mut self, display: &Display) {
        let vertices = glium::VertexBuffer::new(display, &self.mesh.vertices).unwrap();
        let normals = glium::VertexBuffer::new(display, &self.mesh.normals).unwrap();
        let indices = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &self.mesh.indices).unwrap();

        let vertex_shader_src = fs::read_to_string("src/shaders/vertex_shader.glsl").unwrap();
        let fragment_shader_src = fs::read_to_string("src/shaders/fragment_shader.glsl").unwrap();

        let program = Program::from_source(display, &vertex_shader_src, &fragment_shader_src, None).unwrap();

        let model = [
            [0.1, 0.0, 0.0, 0.0],
            [0.0, 0.1, 0.0, 0.0],
            [0.0, 0.0, 0.1, 0.0],
            [0.0, 0.0, 1.0, 1.0f32]
        ];

        let light = [-1.0, 0.4, 0.9f32];

        let mut target = display.draw();
        target.clear_color_and_depth((0.2, 0.2, 0.2, 1.0), 1.0);

        let (width, height) = target.get_dimensions();
        let perspective = self.camera.projection_matrix(width, height);

        let view = self.camera.view_matrix();

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };

        target
            .draw(
                (&vertices, &normals),
                &indices,
                &program,
                &glium::uniform! { model: model, view: view, perspective: perspective, u_light: light },
                &params,
            )
            .unwrap();
        target.finish().unwrap();
    }

    
}

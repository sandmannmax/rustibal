mod camera;
mod maths;
pub mod structures;

use glium::{Display, Program, Surface};
use std::fs;
use structures::Scene;

pub struct Engine {
    camera: camera::Camera,
    scene: Option<Scene>,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            camera: camera::Camera::new(),
            scene: None,
        }
    }

    pub fn set_scene(&mut self, scene: Scene) {
        self.scene = Some(scene);
    }

    pub fn draw(&mut self, display: &Display, pressed: &Vec<String>, released: &Vec<String>) {
        self.camera.set_inputs(pressed, released);
        self.camera.process_inputs();

        let model = [
            [0.1, 0.0, 0.0, 0.0],
            [0.0, 0.1, 0.0, 0.0],
            [0.0, 0.0, 0.1, 0.0],
            [0.0, 0.0, 1.0, 1.0f32],
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
                ..Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise,
            ..Default::default()
        };

        let vertex_shader_src = fs::read_to_string("src/shaders/vertex_shader.glsl").unwrap();
        let fragment_shader_src = fs::read_to_string("src/shaders/fragment_shader.glsl").unwrap();

        let program =
            Program::from_source(display, &vertex_shader_src, &fragment_shader_src, None).unwrap();

        if self.scene.is_some() {
            for mesh in &self.scene.as_ref().unwrap().meshes {
                let vertices = mesh.vertices_buffer(display);
                let indices = mesh.indices_buffer(display);

                target
                    .draw(
                        &vertices,
                        &indices,
                        &program,
                        &glium::uniform! { model: model, view: view, perspective: perspective, u_light: light },
                        &params,
                    )
                    .unwrap();
            }
        }

        target.finish().unwrap();
    }
}

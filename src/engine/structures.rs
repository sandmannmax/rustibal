#[derive(Copy, Clone, Debug)]
pub struct Vertex {
  pub position: (f32, f32, f32),
  pub normal: (f32, f32, f32)
}

glium::implement_vertex!(Vertex, position, normal);

// #[derive(Copy, Clone)]
// pub struct Normal {
// }

// glium::implement_vertex!(Normal, normal);

pub struct Vec3d (
  pub f32,
  pub f32,
  pub f32
);

pub struct Mesh {
  pub vertices: Vec<Vertex>,
  indices: Vec<u32>,
  index_type: glium::index::PrimitiveType
}

impl Mesh {
  pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, index_type: glium::index::PrimitiveType) -> Mesh {
    Mesh {
      vertices,
      indices,
      index_type
    }
  }

  pub fn vertices_buffer(&self, display: &glium::Display) -> glium::VertexBuffer<Vertex> {
    glium::VertexBuffer::new(display, &self.vertices).unwrap()
  }

  // pub fn normals_buffer(&self, display: &glium::Display) -> glium::VertexBuffer<Normal> {
  //   glium::VertexBuffer::new(display, &self.normals).unwrap()
  // }

  pub fn indices_buffer(&self, display: &glium::Display) -> glium::IndexBuffer<u32> {
    glium::IndexBuffer::new(display, self.index_type, &self.indices).unwrap()
  }
}

pub struct Scene {
  pub meshes: Vec<Mesh>
}
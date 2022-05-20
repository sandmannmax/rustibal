#[derive(Copy, Clone)]
pub struct Vertex {
  pub position: (f32, f32, f32)
}

glium::implement_vertex!(Vertex, position);

#[derive(Copy, Clone)]
pub struct Normal {
  pub normal: (f32, f32, f32)
}

glium::implement_vertex!(Normal, normal);

pub struct Vec3d (
  pub f32,
  pub f32,
  pub f32
);

pub struct Mesh {
  pub vertices: Vec<Vertex>,
  pub normals: Vec<Normal>,
  pub indices: Vec<u16>,
}


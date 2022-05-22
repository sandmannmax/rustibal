#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: (f32, f32, f32),
    normal: (f32, f32, f32),
}

impl Vertex {
    pub fn new(position: (f32, f32, f32)) -> Vertex {
        Vertex {
            position,
            normal: (0.0, 0.0, 0.0),
        }
    }

    pub fn new_with_normal(position: (f32, f32, f32), normal: (f32, f32, f32)) -> Vertex {
        Vertex { position, normal }
    }
}

glium::implement_vertex!(Vertex, position, normal);

pub struct Vec3d(pub f32, pub f32, pub f32);

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    indices: Vec<u32>,
    index_type: glium::index::PrimitiveType,
}

impl Mesh {
    pub fn new(
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
        index_type: glium::index::PrimitiveType,
    ) -> Mesh {
        Mesh {
            vertices,
            indices,
            index_type,
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
    pub meshes: Vec<Mesh>,
}

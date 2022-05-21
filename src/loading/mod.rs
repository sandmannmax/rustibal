
use crate::engine::structures::{Vertex, Mesh, Scene};
use crate::utils;
use std::fs;

pub fn load_scene(path: String) -> Scene {
  let mut scene = Scene { meshes: Vec::new() };

  let content = fs::read_to_string(path).unwrap();

  let mut verticesRead = Vec::new();
  let mut normalsRead = Vec::new();
  let mut indicesRead = Vec::new();


  for line in content.split("\n") {
    if line != "" {
      let mut parts = line.split_whitespace();
      let first = parts.next().unwrap();
      let mut partVec = Vec::new();
      for part in parts {
        partVec.push(String::from(part));
      }

      if first == "v" {
        verticesRead.push(load_vertex(partVec));
      } else if first == "vn" {
        normalsRead.push(load_normal(partVec));
      } else if first == "f" {
        let partIndices = load_face(partVec);
        for partIndex in partIndices {
          indicesRead.push(partIndex);
        }
      }
    }
  }

  let mut vertices = Vec::new();
  let mut indices = Vec::new();

  for i in 0..verticesRead.len() {
    vertices.push(Vertex { position: verticesRead[i], normal: normalsRead[i] })
  }

  for i in 0..indicesRead.len() {
    indices.push(indicesRead[i][0]);
  }

  let mesh = Mesh::new(vertices, indices, glium::index::PrimitiveType::TrianglesList);
  scene.meshes.push(mesh);

  scene
}

fn load_vertex(parts: Vec<String>) -> (f32,f32,f32) {  
  if parts.len() != 3 {
    panic!("Length of Vertex must be 3 but was {}", parts.len());
  }

  for i in 0..2 {
    if !utils::can_parse_f32(&parts[i]) {
      panic!("Number can not be parsed: {}", parts[i]);
    }
  }

  // Vertex {
  //   position: (
    (
      parts[0].parse::<f32>().unwrap(),
      parts[1].parse::<f32>().unwrap(),
      parts[2].parse::<f32>().unwrap()
    )
    // normal: (0.0,0.0,0.0)
  // }
}

fn load_normal(parts: Vec<String>) -> (f32,f32,f32) {  
  if parts.len() != 3 {
    panic!("Length of Normal must be 3 but was {}", parts.len());
  }

  for i in 0..2 {
    if !utils::can_parse_f32(&parts[i]) {
      panic!("Number can not be parsed: {}", parts[i]);
    }
  }

  // Normal {
    // normal: (
    (
      parts[0].parse::<f32>().unwrap(),
      parts[1].parse::<f32>().unwrap(),
      parts[2].parse::<f32>().unwrap()
    )
  // }
}

fn load_face(parts: Vec<String>) -> Vec<[u32; 3]> {  
  if parts.len() < 3 {
    panic!("Length of Face can't be less than 3 but was {}", parts.len());
  }

  let mut ps = Vec::new();
  let mut psResult = Vec::new();
  for part in parts {
    ps.push(part.clone());
  }

  for p in ps {
    let mut indexPart: [u32; 3] = [0; 3];
    let parts: Vec<&str> = p.split("/").collect();
    for i in 0..parts.len() {
      if parts[i] == "" {
        continue;
      }
      if !utils::can_parse_u32(&String::from(parts[i])) {
        panic!("Number can not be parsed: {}", parts[i]);
      } else {
        indexPart[i] = parts[i].parse::<u32>().unwrap()-1;
      }
    }
    psResult.push(indexPart);
  }

  return psResult;
}
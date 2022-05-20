
use crate::engine::structures::{Vertex, Normal, Mesh};
use crate::utils;
use std::fs;

pub fn load_obj(path: String) -> Mesh {
  let content = fs::read_to_string(path).unwrap();

  let mut vertices = Vec::new();
  let mut normals = Vec::new();
  let mut indices = Vec::new();

  for line in content.split("\n") {
    if line != "" {
      let mut parts = line.split_whitespace();
      let first = parts.next().unwrap();
      let mut partVec = Vec::new();
      for part in parts {
        partVec.push(String::from(part));
      }

      if first == "v" {
        vertices.push(load_vertex(partVec));
      } else if first == "vn" {
        normals.push(load_normal(partVec));
      } else if first == "f" {
        let (i1, i2, i3) = load_face(partVec);
        indices.push(i1);
        indices.push(i2);
        indices.push(i3);
      }
    }
  }

  let mesh = Mesh {
    vertices,
    normals,
    indices
  };

  mesh
}

fn load_vertex(parts: Vec<String>) -> Vertex {  
  if parts.len() != 3 {
    panic!("Length of Vertex must be 3 but was {}", parts.len());
  }

  for i in 0..2 {
    if !utils::can_parse_f32(&parts[i]) {
      panic!("Number can not be parsed: {}", parts[i]);
    }
  }

  Vertex {
    position: (
      parts[0].parse::<f32>().unwrap(),
      parts[1].parse::<f32>().unwrap(),
      parts[2].parse::<f32>().unwrap()
    )
  }
}

fn load_normal(parts: Vec<String>) -> Normal {  
  if parts.len() != 3 {
    panic!("Length of Normal must be 3 but was {}", parts.len());
  }

  for i in 0..2 {
    if !utils::can_parse_f32(&parts[i]) {
      panic!("Number can not be parsed: {}", parts[i]);
    }
  }

  Normal {
    normal: (
      parts[0].parse::<f32>().unwrap(),
      parts[1].parse::<f32>().unwrap(),
      parts[2].parse::<f32>().unwrap()
    )
  }
}

fn load_face(parts: Vec<String>) -> (u16, u16, u16) {  
  if parts.len() != 3 {
    panic!("Length of Normal must be 3 but was {}", parts.len());
  }

  let mut ps: [String; 3] = [parts[0].clone(), parts[1].clone(), parts[2].clone()];

  if ps[0].contains("//") {
    for i in 0..3 {
      ps[i] = String::from(ps[i].split("//").next().unwrap());
    }
  } else if parts[0].contains("/") {
    for i in 0..3 {
      ps[i] = String::from(ps[i].split("/").next().unwrap());
    }
  }

  for i in 0..3 {
    if !utils::can_parse_u16(&ps[i]) {
      panic!("Number can not be parsed: {}", ps[i]);
    }
  }

  (
    ps[0].parse::<u16>().unwrap(),
    ps[1].parse::<u16>().unwrap(),
    ps[2].parse::<u16>().unwrap()
  ) 
}
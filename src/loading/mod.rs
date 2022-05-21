use crate::engine::structures::{Mesh, Scene, Vertex};
use crate::utils;
use std::fs;

pub fn load_scene(path: String) -> Scene {
    let mut scene = Scene { meshes: Vec::new() };

    let content = fs::read_to_string(path).unwrap();

    let mut vertices_read = Vec::new();
    let mut normals_read = Vec::new();
    let mut indices_read = Vec::new();

    for line in content.split("\n") {
        if line != "" {
            let mut parts = line.split_whitespace();
            let first = parts.next().unwrap();
            let mut part_vec = Vec::new();
            for part in parts {
                part_vec.push(String::from(part));
            }

            if first == "v" {
                vertices_read.push(load_vertex(part_vec));
            } else if first == "vn" {
                normals_read.push(load_normal(part_vec));
            } else if first == "f" {
                let part_indices = load_face(part_vec);
                for part_index in part_indices {
                    indices_read.push(part_index);
                }
            }
        }
    }

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for i in 0..vertices_read.len() {
        vertices.push(Vertex {
            position: vertices_read[i],
            normal: normals_read[i],
        })
    }

    for i in 0..indices_read.len() {
        indices.push(indices_read[i][0]);
    }

    let mesh = Mesh::new(
        vertices,
        indices,
        glium::index::PrimitiveType::TrianglesList,
    );
    scene.meshes.push(mesh);

    scene
}

fn load_vertex(parts: Vec<String>) -> (f32, f32, f32) {
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
        parts[2].parse::<f32>().unwrap(),
    )
    // normal: (0.0,0.0,0.0)
    // }
}

fn load_normal(parts: Vec<String>) -> (f32, f32, f32) {
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
        parts[2].parse::<f32>().unwrap(),
    )
    // }
}

fn load_face(parts: Vec<String>) -> Vec<[u32; 3]> {
    if parts.len() < 3 {
        panic!(
            "Length of Face can't be less than 3 but was {}",
            parts.len()
        );
    }

    let mut ps = Vec::new();
    let mut ps_result = Vec::new();
    for part in parts {
        ps.push(part.clone());
    }

    for p in ps {
        let mut index_part: [u32; 3] = [0; 3];
        let parts: Vec<&str> = p.split("/").collect();
        for i in 0..parts.len() {
            if parts[i] == "" {
                continue;
            }
            if !utils::can_parse_u32(&String::from(parts[i])) {
                panic!("Number can not be parsed: {}", parts[i]);
            } else {
                index_part[i] = parts[i].parse::<u32>().unwrap() - 1;
            }
        }
        ps_result.push(index_part);
    }

    return ps_result;
}

use crate::engine::core::structures::{Mesh, Scene, Vertex};
use crate::utils;
use std::fs;

pub fn load_scene(path: String) -> Scene {
    let mut scene = Scene { meshes: Vec::new() };

    let content = fs::read_to_string(path).unwrap();

    let mut vertices_read = Vec::new();
    let mut normals_read = Vec::new();
    let mut indices_read = Vec::new();
    let mut face_type_read: Option<glium::index::PrimitiveType> = None;

    let mut lines = content.split("\n").collect::<Vec<&str>>();
    clean_lines(&mut lines);

    for line in lines {
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
            if face_type_read.is_none() {
                face_type_read = Some(glium::index::PrimitiveType::TrianglesList);
            }
            for part_index in part_indices {
                indices_read.push(part_index);
            }
        } else if first == "l" {
            let line_indices = load_line(part_vec);
            if face_type_read.is_none() {
                face_type_read = Some(glium::index::PrimitiveType::LinesList);
            }
            for line_index in line_indices {
                indices_read.push(line_index);
            }
        } else if first == "o" {
            if vertices_read.len() != 0 {
                scene.meshes.push(build_mesh(
                    &vertices_read,
                    &normals_read,
                    &indices_read,
                    face_type_read.unwrap(),
                ));
                indices_read = Vec::new();
                face_type_read = None;
            }
        } else if first == "usemtl" || first == "s" {
        } else {
            println!(
                "DEBUG loading: {} is currently not available as identifier",
                first
            );
        }
    }

    if indices_read.len() > 0 {
        scene.meshes.push(build_mesh(
            &vertices_read,
            &normals_read,
            &indices_read,
            face_type_read.unwrap(),
        ));
    }

    scene
}

fn build_mesh(
    vertices_read: &Vec<(f32, f32, f32)>,
    normals_read: &Vec<(f32, f32, f32)>,
    indices_read: &Vec<[u32; 3]>,
    face_type: glium::index::PrimitiveType,
) -> Mesh {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    if normals_read.len() == 0 || face_type == glium::index::PrimitiveType::LinesList {
        for vertex_read in vertices_read {
            vertices.push(Vertex::new(*vertex_read));
        }
        for index_read in indices_read {
            indices.push(index_read[0] - 1);
        }
    } else {
        for i in 0..indices_read.len() {
            if indices_read[i][2] == 0 {
                println!("{} {}", indices_read[i - 1][0], indices_read[i - 1][2]);
                println!("{} {}", indices_read[i][0], indices_read[i][2]);
            }
            let vertex_index = (indices_read[i][0] - 1) as usize;
            let normal_index = (indices_read[i][2] - 1) as usize;
            vertices.push(Vertex::new_with_normal(
                vertices_read[vertex_index],
                normals_read[normal_index],
            ));
            indices.push(i as u32);
        }
    }

    Mesh::new(vertices, indices, face_type)
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
                index_part[i] = parts[i].parse::<u32>().unwrap();
            }
        }
        ps_result.push(index_part);
    }

    if ps_result.len() == 4 {
        ps_result.push(ps_result[2]);
        ps_result.push(ps_result[3]);
        ps_result[3] = ps_result[0];
    }

    return ps_result;
}

fn load_line(parts: Vec<String>) -> Vec<[u32; 3]> {
    if parts.len() != 2 {
        panic!(
            "Length of Line can't be other than 2 but was {}",
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

fn clean_lines(lines: &mut Vec<&str>) {
    let mut removes = Vec::new();
    for i in 0..lines.len() {
        if lines[i].trim() == "" || lines[i].starts_with("#") {
            removes.push(i);
        }
    }

    removes.reverse();

    for remove in removes {
        lines.remove(remove);
    }
}

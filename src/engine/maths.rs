use crate::engine::structures::Vec3d;

pub fn normalize(vec: &Vec3d) -> Vec3d {
    let len = length(vec);
    Vec3d(vec.0 / len, vec.1 / len, vec.2 / len)
}

pub fn length(vec: &Vec3d) -> f32 {
    let len = vec.0 * vec.0 + vec.1 * vec.1 + vec.2 * vec.2;
    len.sqrt()
}

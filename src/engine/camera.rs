use crate::engine::maths;
use crate::engine::structures::Vec3d;

pub struct Camera {
    znear: f32,
    zfar: f32,
    field_of_view: f32,
    position: Vec3d,
    direction: Vec3d,
    up: Vec3d,
    inputs: Vec<String>,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            znear: 0.1,
            zfar: 1024.0,
            field_of_view: 3.141592 / 3.0,
            position: Vec3d(0.0, 0.0, 0.0),
            direction: Vec3d(0.0, 0.0, 1.0),
            up: Vec3d(0.0, 1.0, 0.0),
            inputs: vec![],
        }
    }

    pub fn set_inputs(&mut self, pressed: &Vec<String>, released: &Vec<String>) {
        for press in pressed {
            if !self.inputs.contains(press) {
                self.inputs.push(press.to_string());
            }
        }
        for release in released {
            if self.inputs.contains(release) {
                self.inputs
                    .remove(self.inputs.iter().position(|r| r == release).unwrap());
            }
        }
    }

    pub fn process_inputs(&mut self) {
        for input in self.inputs.clone() {
            if input == String::from("W") {
                self.position.2 += 0.01;
            } else if input == String::from("A") {
                self.position.0 -= 0.01;
            } else if input == String::from("S") {
                self.position.2 -= 0.01;
            } else if input == String::from("D") {
                self.position.0 += 0.01;
            } else if input == String::from("E") {
                self.position.1 += 0.01;
            } else if input == String::from("Q") {
                self.position.1 -= 0.01;
            }
        }
    }

    pub fn projection_matrix(&self, width: u32, height: u32) -> [[f32; 4]; 4] {
        let a = height as f32 / width as f32;
        let e = 1.0 / (self.field_of_view / 2.0).tan();
        let q = self.zfar - self.znear;

        [
            [e * a, 0.0, 0.0, 0.0],
            [0.0, e, 0.0, 0.0],
            [0.0, 0.0, (self.zfar + self.znear) / q, 1.0],
            [0.0, 0.0, -(2.0 * self.zfar * self.znear) / q, 0.0],
        ]
    }

    pub fn view_matrix(&self) -> [[f32; 4]; 4] {
        let normalized_direction =
            maths::normalize(&Vec3d(self.direction.0, self.direction.1, self.direction.2));

        let s = Vec3d(
            self.up.1 * normalized_direction.2 - self.up.2 * normalized_direction.1,
            self.up.2 * normalized_direction.0 - self.up.0 * normalized_direction.2,
            self.up.0 * normalized_direction.1 - self.up.1 * normalized_direction.0,
        );

        let s_norm = maths::normalize(&s);

        let u = [
            normalized_direction.1 * s_norm.2 - normalized_direction.2 * s_norm.1,
            normalized_direction.2 * s_norm.0 - normalized_direction.0 * s_norm.2,
            normalized_direction.0 * s_norm.1 - normalized_direction.1 * s_norm.0,
        ];

        let p = [
            -self.position.0 * s_norm.0 - self.position.1 * s_norm.1 - self.position.2 * s_norm.2,
            -self.position.0 * u[0] - self.position.1 * u[1] - self.position.2 * u[2],
            -self.position.0 * normalized_direction.0
                - self.position.1 * normalized_direction.1
                - self.position.2 * normalized_direction.2,
        ];

        [
            [s_norm.0, u[0], normalized_direction.0, 0.0],
            [s_norm.1, u[1], normalized_direction.1, 0.0],
            [s_norm.2, u[2], normalized_direction.2, 0.0],
            [p[0], p[1], p[2], 1.0],
        ]
    }
}

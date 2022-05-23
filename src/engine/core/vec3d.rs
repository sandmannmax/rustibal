use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vec3d(pub f32, pub f32, pub f32);

impl Vec3d {
    pub fn normalize(&mut self) {
        let len = self.length();
        self.0 /= len;
        self.1 /= len;
        self.2 /= len;
    }

    pub fn length(self) -> f32 {
        let len = self.0 * self.0 + self.1 * self.1 + self.2 * self.2;
        len.sqrt()
    }

    pub fn cross_product(self, vec: Vec3d) -> Vec3d {
        Vec3d(
            self.1 * vec.2 - self.2 * vec.1,
            self.2 * vec.0 - self.0 * vec.2,
            self.0 * vec.1 - self.1 * vec.0,
        )
    }

    pub fn dot_product(self, vec: Vec3d) -> f32 {
        self.0 * vec.0 + self.1 * vec.1 + self.2 * vec.2
    }
}

impl ops::Add<Vec3d> for Vec3d {
    type Output = Vec3d;

    fn add(self, vec: Vec3d) -> Vec3d {
        Vec3d(self.0 + vec.0, self.1 + vec.1, self.2 + vec.2)
    }
}

impl ops::AddAssign<Vec3d> for Vec3d {
    fn add_assign(&mut self, vec: Vec3d) {
        self.0 += vec.0;
        self.1 += vec.1;
        self.2 += vec.2;
    }
}

impl ops::Sub<Vec3d> for Vec3d {
    type Output = Vec3d;

    fn sub(self, vec: Vec3d) -> Vec3d {
        Vec3d(self.0 - vec.0, self.1 - vec.1, self.2 - vec.2)
    }
}

impl ops::SubAssign<Vec3d> for Vec3d {
    fn sub_assign(&mut self, vec: Vec3d) {
        self.0 -= vec.0;
        self.1 -= vec.1;
        self.2 -= vec.2;
    }
}

impl ops::Mul<[[f32; 4]; 4]> for Vec3d {
    type Output = Vec3d;

    fn mul(self, matrix: [[f32; 4]; 4]) -> Vec3d {
        Vec3d(
            matrix[0][0] * self.0 + matrix[1][0] * self.1 + matrix[2][0] * self.2,
            matrix[0][1] * self.0 + matrix[1][1] * self.1 + matrix[2][1] * self.2,
            matrix[0][2] * self.0 + matrix[1][2] * self.1 + matrix[2][2] * self.2,
        )
    }
}

impl ops::Mul<f32> for Vec3d {
    type Output = Vec3d;

    fn mul(self, factor: f32) -> Vec3d {
        Vec3d(self.0 * factor, self.1 * factor, self.2 * factor)
    }
}

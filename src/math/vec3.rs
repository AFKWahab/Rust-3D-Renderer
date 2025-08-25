use std::ops::{Add, Sub, Mul, Div};

#[derive(Copy, Clone)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3f {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3f {
        Self { x, y, z }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Vec3f {
        let len = self.length();
        let x = self.x / len;
        let y = self.y / len;
        let z = self.z / len;
        Vec3f { x, y, z }
    }

    pub fn dot(&self, other: &Vec3f) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3f) -> Vec3f {
        Vec3f {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }


}

impl Add<&Vec3f> for &Vec3f {
    type Output = Vec3f;
    fn add(self, other: &Vec3f) -> Vec3f {
        Vec3f::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub<&Vec3f> for &Vec3f {
    type Output = Vec3f;
    fn sub(self, other: &Vec3f) -> Vec3f {
        Vec3f::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<f32> for &Vec3f {
    type Output = Vec3f;
    fn mul(self, scalar: f32) -> Vec3f {
        Vec3f::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl Div<f32> for &Vec3f {
    type Output = Vec3f;
    fn div(self, scalar: f32) -> Vec3f {
        Vec3f::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}
impl Add for Vec3f {
    type Output = Vec3f;
    fn add(self, other: Vec3f) -> Vec3f {
        Vec3f::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vec3f {
    type Output = Vec3f;
    fn sub(self, other: Vec3f) -> Vec3f {
        Vec3f::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<f32> for Vec3f {
    type Output = Vec3f;
    fn mul(self, scalar: f32) -> Vec3f {
        Vec3f::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl Div<f32> for Vec3f {
    type Output = Vec3f;
    fn div(self, scalar: f32) -> Vec3f {
        Vec3f::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}
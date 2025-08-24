use std::ops::{Add, Sub, Mul, Div};

pub struct Vec3F {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3F {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3F {
        Self { x, y, z }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Vec3F {
        let len = self.length();
        let x = self.x / len;
        let y = self.y / len;
        let z = self.z / len;
        Vec3F { x, y, z }
    }

    pub fn dot(&self, other: &Vec3F) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3F) -> Vec3F {
        Vec3F {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }
}

impl Add for Vec3F {
    type Output = Vec3F;
    fn add(self, other: Vec3F) -> Vec3F {
        Vec3F::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vec3F {
    type Output = Vec3F;
    fn sub(self, other: Vec3F) -> Vec3F {
        Vec3F::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<f32> for Vec3F {
    type Output = Vec3F;
    fn mul(self, other: f32) -> Vec3F {
        Vec3F::new(self.x * other, self.y * other, self.z * other)
    }
}

impl Div<f32> for Vec3F {
    type Output = Vec3F;
    fn div(self, other: f32) -> Vec3F {
        Vec3F::new(self.x / other, self.y / other, self.z / other)
    }
}
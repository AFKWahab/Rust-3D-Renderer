use std::ops::{Add, Sub, Mul, Div};
use crate::math::vec3::Vec3f;

pub struct Vec4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4f {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4f {
        Self { x, y, z, w }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalize(&self) -> Vec4f {
        let len = self.length();
        let x = self.x / len;
        let y = self.y / len;
        let z = self.z / len;
        let w = self.w / len;
        Vec4f { x, y, z, w }
    }

    pub fn dot(&self, other: &Vec4f) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    // Helper functions for homogeneous coordinates
    pub fn from_point(vec3: &Vec3f) -> Vec4f {
        Vec4f::new(vec3.x, vec3.y, vec3.z, 1.0)
    }

    pub fn from_vector(vec3: &Vec3f) -> Vec4f {
        Vec4f::new(vec3.x, vec3.y, vec3.z, 0.0)
    }

    pub fn to_Vec3f(&self) -> Vec3f {
        Vec3f::new(self.x, self.y, self.z)
    }
}

impl Add for Vec4f {
    type Output = Vec4f;
    fn add(self, other: Vec4f) -> Vec4f {
        Vec4f::new(self.x + other.x, self.y + other.y, self.z + other.z, self.w + other.w)
    }
}

impl Sub for Vec4f {
    type Output = Vec4f;
    fn sub(self, other: Vec4f) -> Vec4f {
        Vec4f::new(self.x - other.x, self.y - other.y, self.z - other.z, self.w - other.w)
    }
}

impl Mul<f32> for Vec4f {
    type Output = Vec4f;
    fn mul(self, scalar: f32) -> Vec4f {
        Vec4f::new(self.x * scalar, self.y * scalar, self.z * scalar, self.w * scalar)
    }
}

impl Div<f32> for Vec4f {
    type Output = Vec4f;
    fn div(self, scalar: f32) -> Vec4f {
        Vec4f::new(self.x / scalar, self.y / scalar, self.z / scalar, self.w / scalar)
    }
}
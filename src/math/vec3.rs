use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Copy, Clone, Debug)]
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
        if len > 0.0 {
            Vec3f { x: self.x / len, y: self.y / len, z: self.z / len }
        } else {
            *self
        }
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

    /// Calculate surface normal for a triangle given three vertices
    pub fn calculate_triangle_normal(v0: Vec3f, v1: Vec3f, v2: Vec3f) -> Vec3f {
        let edge1 = v1 - v0;
        let edge2 = v2 - v0;
        edge1.cross(&edge2).normalize()
    }

    /// Zero vector
    pub fn zero() -> Vec3f {
        Vec3f::new(0.0, 0.0, 0.0)
    }

    /// Unit vector pointing up
    pub fn up() -> Vec3f {
        Vec3f::new(0.0, 1.0, 0.0)
    }

    /// Unit vector pointing forward (negative Z)
    pub fn forward() -> Vec3f {
        Vec3f::new(0.0, 0.0, -1.0)
    }

    /// Unit vector pointing right
    pub fn right() -> Vec3f {
        Vec3f::new(1.0, 0.0, 0.0)
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

impl Mul<Vec3f> for Vec3f {
    type Output = Vec3f;
    fn mul(self, other: Vec3f) -> Vec3f {
        Vec3f::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl Neg for Vec3f {
    type Output = Vec3f;
    fn neg(self) -> Vec3f {
        Vec3f::new(-self.x, -self.y, -self.z)
    }
}
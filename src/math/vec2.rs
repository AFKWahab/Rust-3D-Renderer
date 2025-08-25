use std::ops::{Add, Sub, Mul, Div};

pub struct Vec2f {
    pub x: f32,
    pub y: f32
}

impl Vec2f {
    pub fn new(x: f32, y: f32) -> Vec2f {
        Self { x, y }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&self) -> Vec2f {
        let len = self.length();
        let x = self.x / len;
        let y = self.y / len;
        Vec2f { x, y }
    }

    pub fn dot(&self, other: &Vec2f) -> f32 {
        self.x * other.x + self.y * other.y
    }
}



impl Add for Vec2f {
    type Output = Vec2f;
    fn add(self, other: Vec2f) -> Vec2f {
        Vec2f::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Vec2f {
    type Output = Vec2f;
    fn sub(self, other: Vec2f) -> Vec2f {
        Vec2f::new(self.x - other.x, self.y - other.y)
    }
}

impl Mul<f32> for Vec2f {
    type Output = Vec2f;
    fn mul(self, other: f32) -> Vec2f {
        Vec2f::new(self.x * other, self.y * other)
    }
}

impl Div<f32> for Vec2f {
    type Output = Vec2f;
    fn div(self, other: f32) -> Vec2f {
        Vec2f::new(self.x / other, self.y / other)
    }
}
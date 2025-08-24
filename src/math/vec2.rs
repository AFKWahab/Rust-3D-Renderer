use std::ops::{Add, Sub, Mul, Div};

pub struct Vec2F {
    pub x: f32,
    pub y: f32
}

impl Vec2F {
    pub fn new(x: f32, y: f32) -> Vec2F {
        Self { x, y }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&self) -> Vec2F {
        let len = self.length();
        let x = self.x / len;
        let y = self.y / len;
        Vec2F { x, y }
    }

    pub fn dot(&self, other: &Vec2F) -> f32 {
        self.x * other.x + self.y * other.y
    }
}



impl Add for Vec2F {
    type Output = Vec2F;
    fn add(self, other: Vec2F) -> Vec2F {
        Vec2F::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Vec2F {
    type Output = Vec2F;
    fn sub(self, other: Vec2F) -> Vec2F {
        Vec2F::new(self.x - other.x, self.y - other.y)
    }
}

impl Mul<f32> for Vec2F {
    type Output = Vec2F;
    fn mul(self, other: f32) -> Vec2F {
        Vec2F::new(self.x * other, self.y * other)
    }
}

impl Div<f32> for Vec2F {
    type Output = Vec2F;
    fn div(self, other: f32) -> Vec2F {
        Vec2F::new(self.x / other, self.y / other)
    }
}
use crate::math::Vec3f;

#[derive(Copy, Clone)]
pub enum LightType {
    Directional,
    Point,
    Spot { inner_angle: f32, outer_angle: f32}
}
#[derive(Copy, Clone)]
pub struct Light {
    pub light_type: LightType,
    pub position: Vec3f, // For point/spot lights
    pub direction: Vec3f, // For directional/spot lights
    pub color: Vec3f,
    pub intensity: f32,
    pub range: f32, // for point/spot lights
}

impl Light {
    pub fn new(direction: Vec3f, color: Vec3f, intensity: f32) -> Self {
        Self {
            direction: direction.normalize(),
            color,
            intensity,
        }
    }

    pub fn directional(direction: Vec3f) -> Self {
        Self::new(direction, Vec3f::new(1.0, 1.0, 1.0), 1.0)
    }

    pub fn calculate_diffuse(&self, normal: &Vec3f) -> f32 {
        normal.dot(&self.direction).max(0.0) * self.intensity
    }

    pub fn apply_to_color(&self, base_color: u32, intensity: f32, ambient: f32) -> u32 {
        // Extract RGB components
        let r = ((base_color >> 16) & 0xFF) as f32;
        let g = ((base_color >> 8) & 0xFF) as f32;
        let b = (base_color & 0xFF) as f32;

        // Apply light color tinting
        let light_r = r * self.color.x;
        let light_g = g * self.color.y;
        let light_b = b * self.color.z;

        // Combine diffuse lighting with ambient
        let final_intensity = (intensity + ambient).min(1.0);

        println!("Light intensity: {}, ambient: {}, final: {}", intensity, ambient, final_intensity);

        let lit_r = (light_r * final_intensity).min(255.0) as u32;
        let lit_g = (light_g * final_intensity).min(255.0) as u32;
        let lit_b = (light_b * final_intensity).min(255.0) as u32;

        0xFF000000 | (lit_r << 16) | (lit_g << 8) | lit_b
    }
}

pub struct LightingSystem {
    pub lights: Vec<Light>,
    pub ambient_intensity: f32,
}

impl LightingSystem {
    pub fn new() -> Self {
        Self {
            lights: Vec::new(),
            ambient_intensity: 0.05,
        }
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn calculate_lighting(&self, normal: &Vec3f, base_color: u32) -> u32 {
        if self.lights.is_empty() {
            return base_color;
        }

        let mut total_intensity = 0.0;

        // Accumulate lighting from all lights
        for light in &self.lights {
            let diffuse = light.calculate_diffuse(normal);
            total_intensity += diffuse;
        }

        // Use the primary light for color calculation
        let primary_light = &self.lights[0];
        primary_light.apply_to_color(base_color, total_intensity, self.ambient_intensity)
    }
}
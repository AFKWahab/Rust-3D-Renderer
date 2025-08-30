use crate::math::Vec3f;

#[derive(Copy, Clone)]
pub enum LightType {
    Directional,
    Point,
    Spot { inner_angle: f32, outer_angle: f32 },
}

#[derive(Copy, Clone)]
pub struct Light {
    pub light_type: LightType,
    pub position: Vec3f,      // For point/spot lights
    pub direction: Vec3f,     // For directional/spot lights
    pub color: Vec3f,
    pub intensity: f32,
    pub range: f32,           // For point/spot lights
}

impl Light {
    pub fn directional(direction: Vec3f, color: Vec3f, intensity: f32) -> Self {
        Self {
            light_type: LightType::Directional,
            position: Vec3f::zero(),
            direction: direction.normalize(),
            color,
            intensity,
            range: 0.0,
        }
    }

    pub fn point(position: Vec3f, color: Vec3f, intensity: f32, range: f32) -> Self {
        Self {
            light_type: LightType::Point,
            position,
            direction: Vec3f::zero(),
            color,
            intensity,
            range,
        }
    }

    pub fn spot(position: Vec3f, direction: Vec3f, color: Vec3f, intensity: f32,
                range: f32, inner_angle: f32, outer_angle: f32) -> Self {
        Self {
            light_type: LightType::Spot { inner_angle, outer_angle },
            position,
            direction: direction.normalize(),
            color,
            intensity,
            range,
        }
    }

    pub fn calculate_lighting(&self, surface_point: &Vec3f, surface_normal: &Vec3f,
                              view_direction: &Vec3f) -> (f32, f32) {
        let (light_direction, attenuation) = match self.light_type {
            LightType::Directional => {
                // For directional lights, direction is constant and no attenuation
                (-self.direction, 1.0)
            },
            LightType::Point => {
                // Light direction from surface to light
                let light_dir = self.position - *surface_point;
                let distance = light_dir.length();

                if distance > self.range {
                    return (0.0, 0.0);
                }

                let normalized_dir = light_dir.normalize();

                // Distance attenuation (quadratic falloff)
                let attenuation = 1.0 / (1.0 + 0.1 * distance + 0.01 * distance * distance);
                let range_attenuation = ((self.range - distance) / self.range).max(0.0);

                (normalized_dir, attenuation * range_attenuation)
            },
            LightType::Spot { inner_angle, outer_angle } => {
                let light_to_surface = *surface_point - self.position;
                let distance = light_to_surface.length();

                if distance > self.range {
                    return (0.0, 0.0);
                }

                let light_direction = light_to_surface.normalize();
                let spot_direction = self.direction;

                let angle = light_direction.dot(&spot_direction).acos();

                if angle > outer_angle {
                    return (0.0, 0.0);
                }

                // Smooth falloff between inner and outer angles
                let spot_attenuation = if angle < inner_angle {
                    1.0
                } else {
                    ((outer_angle - angle) / (outer_angle - inner_angle)).powf(2.0)
                };

                let distance_attenuation = 1.0 / (1.0 + 0.1 * distance + 0.01 * distance * distance);
                let range_attenuation = ((self.range - distance) / self.range).max(0.0);

                (-light_direction, distance_attenuation * range_attenuation * spot_attenuation)
            }
        };

        if attenuation <= 0.0 {
            return (0.0, 0.0);
        }

        // Diffuse lighting (Lambert)
        let diffuse = surface_normal.dot(&light_direction).max(0.0);

        // Specular lighting (Blinn-Phong)
        let half_vector = (light_direction + *view_direction).normalize();
        let specular_power = 32.0; // Shininess
        let specular = surface_normal.dot(&half_vector).max(0.0).powf(specular_power);

        (diffuse * attenuation, specular * attenuation)
    }
}

pub struct Material {
    pub diffuse_color: Vec3f,
    pub specular_color: Vec3f,
    pub specular_power: f32,
    pub ambient_factor: f32,
}

impl Material {
    pub fn new(diffuse: Vec3f, specular: Vec3f, shininess: f32) -> Self {
        Self {
            diffuse_color: diffuse,
            specular_color: specular,
            specular_power: shininess,
            ambient_factor: 0.1,
        }
    }

    pub fn default() -> Self {
        Self::new(
            Vec3f::new(1.0, 1.0, 1.0),  // White diffuse
            Vec3f::new(0.3, 0.3, 0.3),  // Low specular
            32.0                        // Medium shininess
        )
    }
}

pub struct LightingSystem {
    pub lights: Vec<Light>,
    pub ambient_color: Vec3f,
    pub ambient_intensity: f32,
}

impl LightingSystem {
    pub fn new() -> Self {
        Self {
            lights: Vec::new(),
            ambient_color: Vec3f::new(1.0, 1.0, 1.0),
            ambient_intensity: 0.1,
        }
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn set_ambient(&mut self, color: Vec3f, intensity: f32) {
        self.ambient_color = color;
        self.ambient_intensity = intensity;
    }

    pub fn calculate_lighting(&self, surface_point: &Vec3f, surface_normal: &Vec3f,
                              camera_position: &Vec3f, material: &Material) -> Vec3f {
        // Ambient component
        let ambient = self.ambient_color * self.ambient_intensity * material.ambient_factor;

        let mut final_color = ambient * material.diffuse_color;

        if surface_normal.length() == 0.0 {
            return final_color;
        }

        let view_direction = (*camera_position - *surface_point).normalize();

        // Accumulate lighting from all lights
        for light in &self.lights {
            let (diffuse_intensity, specular_intensity) =
                light.calculate_lighting(surface_point, surface_normal, &view_direction);

            if diffuse_intensity > 0.0 || specular_intensity > 0.0 {
                // Diffuse contribution
                let diffuse_contribution = light.color * light.intensity * diffuse_intensity;
                final_color = final_color + (diffuse_contribution * material.diffuse_color);

                // Specular contribution
                let specular_contribution = light.color * light.intensity * specular_intensity;
                final_color = final_color + (specular_contribution * material.specular_color);
            }
        }

        // Clamp to [0, 1] range
        Vec3f::new(
            final_color.x.min(1.0).max(0.0),
            final_color.y.min(1.0).max(0.0),
            final_color.z.min(1.0).max(0.0),
        )
    }

    pub fn calculate_lighting_u32(&self, surface_point: &Vec3f, surface_normal: &Vec3f,
                                  camera_position: &Vec3f, base_color: u32) -> u32 {
        // Extract base color components
        let base_r = ((base_color >> 16) & 0xFF) as f32 / 255.0;
        let base_g = ((base_color >> 8) & 0xFF) as f32 / 255.0;
        let base_b = (base_color & 0xFF) as f32 / 255.0;

        let material = Material::new(
            Vec3f::new(base_r, base_g, base_b),
            Vec3f::new(0.3, 0.3, 0.3),
            32.0
        );

        let lit_color = self.calculate_lighting(surface_point, surface_normal, camera_position, &material);

        let r = (lit_color.x * 255.0) as u32;
        let g = (lit_color.y * 255.0) as u32;
        let b = (lit_color.z * 255.0) as u32;

        0xFF000000 | (r << 16) | (g << 8) | b
    }
}
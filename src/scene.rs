use crate::math::{Mat4x4, Vec2f, Vec3f};
use crate::mesh::Mesh;
use crate::camera::Camera;
use crate::lighting::{Light, LightingSystem, Material};
use crate::renderer::Renderer;

pub struct GameObject {
    pub mesh: Mesh,
    pub position: Vec3f,
    pub rotation: Vec3f,
    pub scale: Vec3f,
    pub materials: Vec<Material>,
}

impl GameObject {
    pub fn new(mesh: Mesh) -> Self {
        Self {
            mesh,
            position: Vec3f::new(0.0, 0.0, 0.0),
            rotation: Vec3f::new(0.0, 0.0, 0.0),
            scale: Vec3f::new(1.0, 1.0, 1.0),
            materials: vec![Material::default()],
        }
    }

    pub fn with_position(mut self, position: Vec3f) -> Self {
        self.position = position;
        self
    }

    pub fn with_rotation(mut self, rotation: Vec3f) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn with_scale(mut self, scale: Vec3f) -> Self {
        self.scale = scale;
        self
    }

    pub fn add_material(&mut self, material: Material) -> usize {
        self.materials.push(material);
        self.materials.len() - 1
    }

    pub fn get_model_matrix(&self) -> Mat4x4 {
        let translation = Mat4x4::translation(self.position.x, self.position.y, self.position.z);
        let rotation_x = Mat4x4::rotation_x(self.rotation.x);
        let rotation_y = Mat4x4::rotation_y(self.rotation.y);
        let rotation_z = Mat4x4::rotation_z(self.rotation.z);
        let scale = Mat4x4::scale(self.scale.x, self.scale.y, self.scale.z);

        translation.multiply(&rotation_z.multiply(&rotation_y.multiply(&rotation_x.multiply(&scale))))
    }

    pub fn get_normal_matrix(&self) -> Mat4x4 {
        // For normal transformation, we need inverse transpose of upper 3x3 of model matrix
        // For uniform scaling and rotation, we can use the model matrix directly
        // For non-uniform scaling, we'd need proper inverse transpose
        self.get_model_matrix()
    }
}

pub struct Scene {
    pub game_objects: Vec<GameObject>,
    pub camera: Camera,
    pub lighting: LightingSystem,
    pub rotation_time: f32,
}

impl Scene {
    pub fn new() -> Self {
        let mut lighting = LightingSystem::new();

        // Set up better ambient lighting
        lighting.set_ambient(Vec3f::new(0.4, 0.4, 0.5), 0.15);

        Self {
            game_objects: Vec::new(),
            camera: Camera::look_at(
                Vec3f::new(0.0, 0.0, 8.0),  // Move camera further back
                Vec3f::new(0.0, 0.0, 0.0),
                Vec3f::new(0.0, 1.0, 0.0),
            ),
            lighting,
            rotation_time: 0.0,
        }
    }

    pub fn add_game_object(&mut self, game_object: GameObject) {
        self.game_objects.push(game_object);
    }

    pub fn render(&mut self, renderer: &mut Renderer) {
        renderer.clear(0xFF111111); // Dark gray background

        // Update camera aspect ratio
        let (width, height) = renderer.get_dimension();
        self.camera.set_aspect_ratio(width as f32, height as f32);

        let view_matrix = self.camera.get_view_matrix();
        let proj_matrix = self.camera.get_projection_matrix();

        // Render all game objects
        for game_object in &self.game_objects {
            self.render_game_object(game_object, &view_matrix, &proj_matrix, renderer);
        }
    }

    fn render_game_object(&self, game_object: &GameObject, view_matrix: &Mat4x4,
                          proj_matrix: &Mat4x4, renderer: &mut Renderer) {
        let model_matrix = game_object.get_model_matrix();
        let normal_matrix = game_object.get_normal_matrix();

        // Transform vertices to world space
        let world_vertices = game_object.mesh.transform_vertices(&model_matrix);

        // Transform normals to world space
        let world_normals = game_object.mesh.transform_normals(&normal_matrix);

        // Process each triangle
        for (triangle_index, triangle) in game_object.mesh.triangles.iter().enumerate() {
            let (v0_world, v1_world, v2_world) = (
                world_vertices[triangle.indices[0]],
                world_vertices[triangle.indices[1]],
                world_vertices[triangle.indices[2]],
            );

            let world_normal = if triangle_index < world_normals.len() {
                world_normals[triangle_index]
            } else {
                Vec3f::calculate_triangle_normal(v0_world, v1_world, v2_world)
            };

            // Backface culling
            let triangle_center = Vec3f::new(
                (v0_world.x + v1_world.x + v2_world.x) / 3.0,
                (v0_world.y + v1_world.y + v2_world.y) / 3.0,
                (v0_world.z + v1_world.z + v2_world.z) / 3.0,
            );

            let view_direction = (self.camera.position - triangle_center).normalize();
            if world_normal.dot(&view_direction) < 0.0 {
                continue; // Skip back-facing triangles
            }

            // Transform to camera space
            let v0_camera = view_matrix.multiply_point(&v0_world);
            let v1_camera = view_matrix.multiply_point(&v1_world);
            let v2_camera = view_matrix.multiply_point(&v2_world);

            // Skip if triangle is behind camera
            if v0_camera.z >= 0.0 || v1_camera.z >= 0.0 || v2_camera.z >= 0.0 {
                continue;
            }

            // Project to screen space
            if let (Some(screen0), Some(screen1), Some(screen2)) = (
                self.project_to_screen(&v0_camera, proj_matrix, renderer),
                self.project_to_screen(&v1_camera, proj_matrix, renderer),
                self.project_to_screen(&v2_camera, proj_matrix, renderer),
            ) {
                // Calculate lighting
                let material = game_object.materials.get(
                    triangle.material_id.unwrap_or(0)
                ).unwrap_or(&game_object.materials[0]);

                let lit_color = self.lighting.calculate_lighting(
                    &triangle_center,
                    &world_normal,
                    &self.camera.position,
                    material
                );

                // Convert to u32 color
                let final_color = self.vec3_to_color(lit_color);

                // Convert camera Z to normalized depth for z-buffer
                let z0 = -v0_camera.z / 100.0; // Normalize by far plane distance
                let z1 = -v1_camera.z / 100.0;
                let z2 = -v2_camera.z / 100.0;

                renderer.draw_triangle(screen0, screen1, screen2, z0, z1, z2, final_color);
            }
        }
    }

    fn project_to_screen(&self, camera_point: &Vec3f, proj_matrix: &Mat4x4,
                         renderer: &Renderer) -> Option<Vec2f> {
        if camera_point.z >= 0.0 {
            return None;
        }

        let projected_4d = proj_matrix.multiply_point_4d(camera_point);

        if projected_4d.w == 0.0 {
            return None;
        }

        let ndc_x = projected_4d.x / projected_4d.w;
        let ndc_y = projected_4d.y / projected_4d.w;

        // Check if point is within NDC bounds
        if ndc_x < -1.0 || ndc_x > 1.0 || ndc_y < -1.0 || ndc_y > 1.0 {
            return None;
        }

        let (width, height) = renderer.get_dimension();
        let pixel_x = (ndc_x + 1.0) * 0.5 * width as f32;
        let pixel_y = (1.0 - ndc_y) * 0.5 * height as f32;

        Some(Vec2f::new(pixel_x, pixel_y))
    }

    fn vec3_to_color(&self, color: Vec3f) -> u32 {
        let r = (color.x.min(1.0).max(0.0) * 255.0) as u32;
        let g = (color.y.min(1.0).max(0.0) * 255.0) as u32;
        let b = (color.z.min(1.0).max(0.0) * 255.0) as u32;

        0xFF000000 | (r << 16) | (g << 8) | b
    }

    // Utility methods
    pub fn add_cube_at(&mut self, position: Vec3f) {
        let cube_mesh = Mesh::create_cube();
        let mut cube_object = GameObject::new(cube_mesh).with_position(position);

        // Add some interesting materials
        let shiny_material = Material::new(
            Vec3f::new(0.8, 0.2, 0.2), // Red diffuse
            Vec3f::new(0.9, 0.9, 0.9), // High specular
            128.0                      // Very shiny
        );
        cube_object.add_material(shiny_material);

        self.add_game_object(cube_object);
    }

    pub fn add_triangle_at(&mut self, position: Vec3f) {
        let triangle_mesh = Mesh::create_triangle();
        let triangle_object = GameObject::new(triangle_mesh).with_position(position);
        self.add_game_object(triangle_object);
    }

    pub fn set_camera_position(&mut self, position: Vec3f) {
        self.camera.position = position;
    }

    pub fn set_camera_target(&mut self, target: Vec3f) {
        self.camera.target = target;
    }

    pub fn add_light(&mut self, light: Light) {
        self.lighting.add_light(light);
    }

    pub fn update(&mut self, delta_time: f32) {
        self.rotation_time += delta_time;

        // Rotate cubes
        for (i, game_object) in self.game_objects.iter_mut().enumerate() {
            let offset = i as f32 * 0.5;
            game_object.rotation.y = self.rotation_time + offset;
            game_object.rotation.x = self.rotation_time * 0.3 + offset;
        }
    }
}
use crate::math::{Mat4x4, Vec2f, Vec3f, Vec4f};
use crate::mesh::{Mesh, Triangle};
use crate::camera::Camera;
use crate::lighting::LightingSystem;
use crate::renderer::Renderer;

pub struct GameObject {
    pub mesh: Mesh,
    pub position: Vec3f,
    pub rotation: Vec3f,
    pub scale: Vec3f,
}

impl GameObject {
    pub fn new(mesh: Mesh) -> Self {
        Self {
            mesh,
            position: Vec3f::new(0.0, 0.0, 0.0),
            rotation: Vec3f::new(0.0, 0.0, 0.0),
            scale: Vec3f::new(1.0, 1.0, 1.0),
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

    pub fn get_model_matrix(&self) -> Mat4x4 {
        let translation = Mat4x4::translation(self.position.x, self.position.y, self.position.z);
        let rotation_x = Mat4x4::rotation_x(self.rotation.x);
        let rotation_y = Mat4x4::rotation_y(self.rotation.y);
        let rotation_z = Mat4x4::rotation_z(self.rotation.z);
        let scale = Mat4x4::scale(self.scale.x, self.scale.y, self.scale.z);

        translation.multiply(&rotation_z.multiply(&rotation_y.multiply(&rotation_x.multiply(&scale))))
    }
}

pub struct Scene {
    pub game_objects_count: usize, // Simplified for debugging
    pub camera: Camera,
    pub lighting: LightingSystem,
}

impl Scene {
    pub fn new() -> Self {
        let mut lighting = LightingSystem::new();
        lighting.add_light(crate::lighting::Light::directional(
            Vec3f::new(1.0, 1.0, 1.0).normalize()
        ));

        Self {
            game_objects_count: 0,
            camera: Camera::look_at(
                Vec3f::new(0.0, 0.0, 5.0),  // Move camera further back
                Vec3f::new(0.0, 0.0, 0.0),
                Vec3f::new(0.0, 1.0, 0.0),
            ),
            lighting,
        }
    }

    // Simplified for debugging - we'll hard-code one cube
    pub fn add_game_object(&mut self, _game_object: GameObject) {
        self.game_objects_count += 1;
    }

    pub fn render(&self, renderer: &mut Renderer) {
        renderer.clear(0xFF000000);

        println!("Starting render...");

        // STEP 1: Test with a simple screen-space triangle
        println!("Drawing test triangle...");
        let v0 = Vec2f::new(400.0, 100.0);  // Top center
        let v1 = Vec2f::new(200.0, 400.0);  // Bottom left
        let v2 = Vec2f::new(600.0, 400.0);  // Bottom right

        renderer.draw_triangle(v0, v1, v2, 1.0, 1.0, 1.0, 0xFF00FF00); // Bright green

        // STEP 2: If triangle appears, test 3D cube
        if self.game_objects_count > 0 {
            self.render_3d_cube(renderer);
        }

        println!("Render complete.");
    }

    fn render_3d_cube(&self, renderer: &mut Renderer) {
        println!("Rendering 3D cube...");

        let view_matrix = self.camera.get_view_matrix();
        let proj_matrix = self.camera.get_projection_matrix();

        // Hard-coded simple cube
        let cube_vertices = [
            Vec3f::new(-0.5, -0.5,  0.5), // Front face
            Vec3f::new( 0.5, -0.5,  0.5),
            Vec3f::new( 0.5,  0.5,  0.5),
            Vec3f::new(-0.5,  0.5,  0.5),
            Vec3f::new(-0.5, -0.5, -0.5), // Back face
            Vec3f::new( 0.5, -0.5, -0.5),
            Vec3f::new( 0.5,  0.5, -0.5),
            Vec3f::new(-0.5,  0.5, -0.5),
        ];

        // Transform to camera space and project
        let mut screen_vertices = Vec::new();
        let mut camera_vertices = Vec::new();

        for vertex in &cube_vertices {
            let camera_point = view_matrix.multiply_point(vertex);  // Fixed: removed .inverse()

            if camera_point.z <= 0.0 {
                println!("Vertex behind camera, skipping cube");
                return;
            }

            if let Some(screen_pos) = self.project_to_screen(&camera_point, renderer) {
                screen_vertices.push(screen_pos);
                camera_vertices.push(camera_point);
            } else {
                println!("Projection failed, skipping cube");
                return;
            }
        }

        // Draw just the front face for now
        let triangles = [
            (0, 1, 2, 0xFFFF0000), // Bright red
            (2, 3, 0, 0xFF0000FF), // Bright blue
        ];

        for (i0, i1, i2, color) in &triangles {
            let v0 = screen_vertices[*i0];
            let v1 = screen_vertices[*i1];
            let v2 = screen_vertices[*i2];

            let z0 = camera_vertices[*i0].z;
            let z1 = camera_vertices[*i1].z;
            let z2 = camera_vertices[*i2].z;

            println!("Drawing triangle: {:?} {:?} {:?} with color {:08X}", v0, v1, v2, color);
            renderer.draw_triangle(v0, v1, v2, z0, z1, z2, *color);
        }
    }

    fn project_to_screen(&self, camera_point: &Vec3f, renderer: &Renderer) -> Option<Vec2f> {
        if camera_point.z <= 0.0 {
            return None;
        }

        // Apply perspective projection matrix using the new method
        let proj_matrix = self.camera.get_projection_matrix();
        let projected_4d = proj_matrix.multiply_point_4d(camera_point);

        // Perspective divide
        if projected_4d.w == 0.0 {
            return None;
        }

        let ndc_x = projected_4d.x / projected_4d.w;
        let ndc_y = projected_4d.y / projected_4d.w;

        // Convert to screen coordinates
        let (width, height) = renderer.get_dimension();
        let pixel_x = (ndc_x + 1.0) * 0.5 * width as f32;
        let pixel_y = (1.0 - ndc_y) * 0.5 * height as f32;

        Some(Vec2f::new(pixel_x, pixel_y))
    }

    // Utility methods
    pub fn add_cube_at(&mut self, _position: Vec3f) {
        self.game_objects_count += 1;
        println!("Added cube (total: {})", self.game_objects_count);
    }

    pub fn add_triangle_at(&mut self, _position: Vec3f) {
        self.game_objects_count += 1;
    }

    pub fn set_camera_position(&mut self, position: Vec3f) {
        self.camera.position = position;
        println!("Camera position set to: {:?}", position);
    }

    pub fn set_camera_target(&mut self, target: Vec3f) {
        self.camera.target = target;
        println!("Camera target set to: {:?}", target);
    }

    pub fn add_light(&mut self, light: crate::lighting::Light) {
        self.lighting.add_light(light);
    }

    pub fn update(&mut self, _delta_time: f32) {
        // No rotation for debugging
    }
}
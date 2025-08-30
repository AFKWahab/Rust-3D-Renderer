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
        let lighting = LightingSystem::new();
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

        let cube_vertices = [
            Vec3f::new(-0.5, -0.5,  0.5), // 0: Front face
            Vec3f::new( 0.5, -0.5,  0.5), // 1
            Vec3f::new( 0.5,  0.5,  0.5), // 2
            Vec3f::new(-0.5,  0.5,  0.5), // 3
            Vec3f::new(-0.5, -0.5, -0.5), // 4: Back face
            Vec3f::new( 0.5, -0.5, -0.5), // 5
            Vec3f::new( 0.5,  0.5, -0.5), // 6
            Vec3f::new(-0.5,  0.5, -0.5), // 7
        ];

        // Transform vertices to camera space and project to screen
        let mut screen_vertices = Vec::new();
        let mut depths = Vec::new();

        for (i, vertex) in cube_vertices.iter().enumerate() {
            let camera_point = view_matrix.multiply_point(vertex);

            if camera_point.z >= 0.0 {
                println!("Vertex {} behind camera, skipping cube", i);
                return;
            }

            let projected_4d = proj_matrix.multiply_point_4d(&camera_point);

            if projected_4d.w == 0.0 {
                return;
            }

            let ndc_x = projected_4d.x / projected_4d.w;
            let ndc_y = projected_4d.y / projected_4d.w;
            let ndc_z = projected_4d.z / projected_4d.w;

            let (width, height) = renderer.get_dimension();
            let pixel_x = (ndc_x + 1.0) * 0.5 * width as f32;
            let pixel_y = (1.0 - ndc_y) * 0.5 * height as f32;

            screen_vertices.push(Vec2f::new(pixel_x, pixel_y));
            depths.push(ndc_z);
        }

        // Define triangles with their base colors (before lighting)
        let triangles = [
            (0, 1, 2, 0xFFFFFFFF), // White base color
            (2, 3, 0, 0xFFFFFFFF), // White base color
        ];

        for (i0, i1, i2, base_color) in &triangles {
            // Calculate triangle normal in world space
            let v0_world = cube_vertices[*i0];
            let v1_world = cube_vertices[*i1];
            let v2_world = cube_vertices[*i2];

            let normal = Vec3f::calculate_triangle_normal(v0_world, v1_world, v2_world);
            println!("Triangle normal: {:?}", normal);

            // Apply lighting to get final color
            let lit_color = self.lighting.calculate_lighting(&normal, *base_color);
            println!("Base color: {:08X} -> Lit color: {:08X}", base_color, lit_color);

            // Draw the triangle with lit color
            let v0_screen = screen_vertices[*i0];
            let v1_screen = screen_vertices[*i1];
            let v2_screen = screen_vertices[*i2];

            let z0 = depths[*i0];
            let z1 = depths[*i1];
            let z2 = depths[*i2];

            renderer.draw_triangle(v0_screen, v1_screen, v2_screen, z0, z1, z2, lit_color);
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
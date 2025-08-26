use crate::math::{Mat4x4, Vec2f, Vec3f};
use crate::mesh::{Mesh, Triangle};
use crate::camera::Camera;
use crate::lighting::LightingSystem;
use crate::renderer::Renderer;

pub struct GameObject {
    pub mesh: Mesh,
    pub position: Vec3f,
    pub rotation: Vec3f, // Euler angles in radians
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

        // Apply transformations: Translation * Rotation * Scale
        translation.multiply(&rotation_z.multiply(&rotation_y.multiply(&rotation_x.multiply(&scale))))
    }
}

pub struct Scene {
    pub game_objects: Vec<GameObject>,
    pub camera: Camera,
    pub lighting: LightingSystem,
}

impl Scene {
    pub fn new() -> Self {
        let mut lighting = LightingSystem::new();
        // Add default light
        lighting.add_light(crate::lighting::Light::directional(
            Vec3f::new(1.0, 1.0, 1.0).normalize()
        ));

        Self {
            game_objects: Vec::new(),
            camera: Camera::look_at(
                Vec3f::new(0.0, 2.0, 5.0),
                Vec3f::new(0.0, 0.0, 0.0),
                Vec3f::new(0.0, 1.0, 0.0),
            ),
            lighting,
        }
    }

    pub fn add_game_object(&mut self, game_object: GameObject) {
        self.game_objects.push(game_object);
    }

    pub fn render(&self, renderer: &mut Renderer) {
        renderer.clear(0xFF000000);

        let view_matrix = self.camera.get_view_matrix();
        let (width, height) = renderer.get_dimension();

        // Update camera aspect ratio
        let mut camera = self.camera;
        camera.set_aspect_ratio(width as f32, height as f32);

        for game_object in &self.game_objects {
            self.render_game_object(game_object, &view_matrix, renderer);
        }
    }

    fn render_game_object(&self, game_object: &GameObject, view_matrix: &Mat4x4, renderer: &mut Renderer) {
        let model_matrix = game_object.get_model_matrix();
        let model_view_matrix = view_matrix.multiply(&model_matrix);

        // Transform all vertices to camera space
        let transformed_vertices = game_object.mesh.transform_vertices(&model_view_matrix);

        // Project to screen coordinates
        let mut screen_vertices = Vec::new();
        let mut camera_vertices = Vec::new();

        for vertex in &transformed_vertices {
            if let Some(screen_pos) = self.project_to_screen(vertex, renderer) {
                screen_vertices.push(screen_pos);
                camera_vertices.push(*vertex);
            } else {
                return; // Skip if any vertex is behind camera
            }
        }

        // Render all triangles
        for triangle in &game_object.mesh.triangles {
            self.render_triangle(
                triangle,
                &game_object.mesh,
                &screen_vertices,
                &camera_vertices,
                &model_matrix,
                renderer,
            );
        }
    }

    fn render_triangle(
        &self,
        triangle: &Triangle,
        mesh: &Mesh,
        screen_vertices: &[Vec2f],
        camera_vertices: &[Vec3f],
        model_matrix: &Mat4x4,
        renderer: &mut Renderer,
    ) {
        let i0 = triangle.indices[0];
        let i1 = triangle.indices[1];
        let i2 = triangle.indices[2];

        let screen_v0 = screen_vertices[i0];
        let screen_v1 = screen_vertices[i1];
        let screen_v2 = screen_vertices[i2];

        let z0 = camera_vertices[i0].z;
        let z1 = camera_vertices[i1].z;
        let z2 = camera_vertices[i2].z;

        // Calculate world-space normal
        let world_v0 = model_matrix.multiply_point(&mesh.vertices[i0]);
        let world_v1 = model_matrix.multiply_point(&mesh.vertices[i1]);
        let world_v2 = model_matrix.multiply_point(&mesh.vertices[i2]);
        let normal = Vec3f::calculate_triangle_normal(world_v0, world_v1, world_v2);

        // Apply lighting
        let lit_color = self.lighting.calculate_lighting(&normal, triangle.color);

        // Render the triangle
        renderer.draw_triangle(screen_v0, screen_v1, screen_v2, z0, z1, z2, lit_color);
    }

    fn project_to_screen(&self, camera_point: &Vec3f, renderer: &Renderer) -> Option<Vec2f> {
        // Simple perspective projection
        if camera_point.z <= 0.0 {
            return None; // Behind camera
        }

        let screen_x = camera_point.x / camera_point.z;
        let screen_y = camera_point.y / camera_point.z;

        let (width, height) = renderer.get_dimension();
        let pixel_x = ((screen_x + 1.0) * 0.5 * width as f32) as i32;
        let pixel_y = ((1.0 - screen_y) * 0.5 * height as f32) as i32;

        Some(Vec2f::new(pixel_x as f32, pixel_y as f32))
    }

    // Utility methods for scene manipulation
    pub fn add_cube_at(&mut self, position: Vec3f) {
        let cube = GameObject::new(Mesh::create_cube()).with_position(position);
        self.add_game_object(cube);
    }

    pub fn add_triangle_at(&mut self, position: Vec3f) {
        let triangle = GameObject::new(Mesh::create_triangle()).with_position(position);
        self.add_game_object(triangle);
    }

    pub fn set_camera_position(&mut self, position: Vec3f) {
        self.camera.position = position;
    }

    pub fn set_camera_target(&mut self, target: Vec3f) {
        self.camera.target = target;
    }

    pub fn add_light(&mut self, light: crate::lighting::Light) {
        self.lighting.add_light(light);
    }

    pub fn update(&mut self, _delta_time: f32) {
        // TODO
        // This is where we would add animations, physics etc.
        // This would be stuff such as
        // - Rotate objects
        // - Move camera
        // - Update particle systems
        // - Handle collision detection

        // Simple rotation example:
        for game_object in &mut self.game_objects {
            game_object.rotation.y += 0.01; // Rotate around Y axis
        }
    }
}
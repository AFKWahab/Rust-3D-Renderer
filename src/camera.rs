use crate::math::{Mat4x4, Vec3f};

#[derive(Copy, Clone)]
pub struct Camera {
    pub position: Vec3f,
    pub target: Vec3f,
    pub up: Vec3f,
    pub fov: f32,        // Field of view in radians
    pub aspect: f32,     // Width / Height ratio
    pub near: f32,       // Near clipping plane
    pub far: f32,        // Far clipping plane
}

impl Camera {
    pub fn new(position: Vec3f, target: Vec3f, up: Vec3f) -> Self {
        Self {
            position,
            target,
            up: up.normalize(),
            fov: std::f32::consts::PI / 4.0, // 45 degrees
            aspect: 4.0 / 3.0,                // 4:3 aspect ratio
            near: 0.1,
            far: 100.0,
        }
    }

    pub fn look_at(eye: Vec3f, target: Vec3f, up: Vec3f) -> Self {
        Self::new(eye, target, up)
    }

    pub fn get_view_matrix(&self) -> Mat4x4 {
        Mat4x4::look_at(self.position, self.target, self.up)
    }

    pub fn get_projection_matrix(&self) -> Mat4x4 {
        Mat4x4::perspective(self.fov, self.aspect, self.near, self.far)
    }

    pub fn set_aspect_ratio(&mut self, width: f32, height: f32) {
        self.aspect = width / height;
    }

    // Camera movement methods
    pub fn move_forward(&mut self, distance: f32) {
        let forward = (self.target - self.position).normalize();
        self.position = self.position + forward * distance;
        self.target = self.target + forward * distance;
    }

    pub fn move_right(&mut self, distance: f32) {
        let forward = (self.target - self.position).normalize();
        let right = forward.cross(&self.up).normalize();
        self.position = self.position + right * distance;
        self.target = self.target + right * distance;
    }

    pub fn move_up(&mut self, distance: f32) {
        self.position = self.position + self.up * distance;
        self.target = self.target + self.up * distance;
    }

    pub fn rotate_around_target(&mut self, yaw: f32, pitch: f32) {
        // Calculate current direction from position to target
        let direction = self.target - self.position;
        let distance = direction.length();

        // Convert to spherical coordinates
        let current_yaw = direction.z.atan2(direction.x);
        let current_pitch = (direction.y / distance).asin();

        // Apply rotation
        let new_yaw = current_yaw + yaw;
        let new_pitch = (current_pitch + pitch).max(-std::f32::consts::PI / 2.1).min(std::f32::consts::PI / 2.1);

        // Convert back to cartesian
        let new_direction = Vec3f::new(
            distance * new_pitch.cos() * new_yaw.cos(),
            distance * new_pitch.sin(),
            distance * new_pitch.cos() * new_yaw.sin(),
        );

        self.position = self.target - new_direction;
    }

    pub fn orbit_around_point(&mut self, center: Vec3f, yaw: f32, pitch: f32) {
        let old_target = self.target;
        self.target = center;
        self.rotate_around_target(yaw, pitch);
        self.target = old_target;
    }

    pub fn look_in_direction(&mut self, direction: Vec3f) {
        self.target = self.position + direction.normalize();
    }

    pub fn get_forward_vector(&self) -> Vec3f {
        (self.target - self.position).normalize()
    }

    pub fn get_right_vector(&self) -> Vec3f {
        self.get_forward_vector().cross(&self.up).normalize()
    }

    pub fn get_up_vector(&self) -> Vec3f {
        self.up
    }
}
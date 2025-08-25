use crate::math::{Mat4x4, Vec2f, Vec3f};

pub struct Renderer {
    width: u32,
    height: u32,
    framebuffer: Vec<u32>, // ARGB Pixels
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            framebuffer: vec![0xFF000000; (width * height) as usize],
        }
    }

    pub fn project_to_screen(&self, world_point: &Vec3f, camera_matrix: &Mat4x4) -> Option<Vec2f> {
        // Step 1: Transform world point to camera space
        let camera_point = camera_matrix.inverse()?.multiply_point(&world_point);

        // Step 2: Perspective projection (3D → 2D)
        if camera_point.z <= 0.0 { return None; } // Behind camera
        let screen_x = camera_point.x / camera_point.z;
        let screen_y = camera_point.y / camera_point.z;

        // Step 3: Convert to pixel coordinates
        let pixel_x = ((screen_x + 1.0) * 0.5 * self.width as f32) as i32;
        let pixel_y = ((1.0 - screen_y) * 0.5 * self.height as f32) as i32;

        Some(Vec2f::new(pixel_x as f32, pixel_y as f32))
    }

    // We do &mut self, because wew ant to borrow the Renderer, but we also modify the framebuffer, so we need mutable access
    pub fn render_frame(&mut self) {
        self.clear(0xFF000000);

        // Create a camera
        let camera = Mat4x4::look_at(
            Vec3f::new(0.0, 2.0, 5.0),   // Camera position
            Vec3f::new(0.0, 0.0, 0.0),   // Look at origin
            Vec3f::new(0.0, 1.0, 0.0)    // Up vector
        );

        // Define a COMPLETE 3D cube (8 vertices)
        let cube_vertices = [
            // Front face
            Vec3f::new(-1.0, -1.0,  1.0), // 0: bottom-left-front
            Vec3f::new( 1.0, -1.0,  1.0), // 1: bottom-right-front
            Vec3f::new( 1.0,  1.0,  1.0), // 2: top-right-front
            Vec3f::new(-1.0,  1.0,  1.0), // 3: top-left-front
            // Back face
            Vec3f::new(-1.0, -1.0, -1.0), // 4: bottom-left-back
            Vec3f::new( 1.0, -1.0, -1.0), // 5: bottom-right-back
            Vec3f::new( 1.0,  1.0, -1.0), // 6: top-right-back
            Vec3f::new(-1.0,  1.0, -1.0), // 7: top-left-back
        ];

        // Project all vertices to screen coordinates
        let mut screen_vertices = Vec::new();
        for vertex in &cube_vertices {
            if let Some(screen_pos) = self.project_to_screen(vertex, &camera) {
                screen_vertices.push(screen_pos);
            } else {
                return; // Skip if any vertex is behind camera
            }
        }

        // Draw the cube edges (12 edges total)
        let edges = [
            // Front face edges
            (0, 1), (1, 2), (2, 3), (3, 0),
            // Back face edges
            (4, 5), (5, 6), (6, 7), (7, 4),
            // Connecting front to back
            (0, 4), (1, 5), (2, 6), (3, 7),
        ];

        // Draw lines between connected vertices
        for (start, end) in &edges {
            let start_pos = &screen_vertices[*start];
            let end_pos = &screen_vertices[*end];

            self.draw_line(
                start_pos.x as i32, start_pos.y as i32,
                end_pos.x as i32, end_pos.y as i32,
                0xFF00FF00 // Green lines
            );
        }
    }

    //pub fn project_to_screen(&self, world_point: Vec3F, camera_pos: Vec3F) -> Self {

    //}
    // Bresenhams line algorithmn
    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: u32) {
        let dx= (x1 - x0).abs();
        let dy = (y1 - y0).abs();

        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };

        let mut err = dx - dy;

        let mut x = x0;
        let mut y = y0;
        loop {
            self.set_pixel(x as u32, y as u32, color);

            if x == x1 && y == y1 { break; }

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }

            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }
    pub fn get_framebuffer(&self) -> &[u32] {
            &self.framebuffer
    }

    pub fn get_dimension(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn clear(&mut self, color: u32) {
        for pixel in &mut self.framebuffer {
            *pixel = color;
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: u32) {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize;
            self.framebuffer[index] = color;
        }
    }
}
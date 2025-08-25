use crate::math::{Mat4x4, Vec2f, Vec3f};

pub struct Renderer {
    width: u32,
    height: u32,
    framebuffer: Vec<u32>, // ARGB Pixels
    z_buffer: Vec<f32>,
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            framebuffer: vec![0xFF000000; (width * height) as usize],
            z_buffer: vec![f32::INFINITY; (width * height) as usize]
        }
    }


    ///
    /// Given triangle with vertices A, B, C and point P, we want to find weights (u,v,w) such that
    /// P=u*A + v*B + w*C
    /// u v + w = 1
    fn barycentric_coordinates(&self, p: Vec2f, v0: Vec2f, v1: Vec2f, v2: Vec2f) -> (f32, f32, f32) {
        // TODO -> Barycentric coordinate calculation here
        let v0v1 = Vec2f::new(v1.x - v0.x, v1.y - v0.y);
        let v0v2 = Vec2f::new(v2.x - v0.x, v2.y - v0.y);
        let v0p = Vec2f::new(p.x - v0.x, p.y - v0.y);

        let dot00 = v0v2.dot(&v0v2); // v0v2 · v0v2
        let dot01 = v0v1.dot(&v0v2); // v0v2 · v0v1
        let dot02 = v0v2.dot(&v0p); // v0v2 · v0p
        let dot11 = v0v1.dot(&v0v1); // v0v1 · v0v1
        let dot12 = v0v1.dot(&v0p); // v0v1 · v0p

        // Calculating barycentric coordinates
        let inv_denom =  1.0 / (dot00 * dot11 - dot01 * dot01);
        let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
        let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;
        let w = 1.0 - u - v;

        (u, v, w) // We return this because its the weights
    }
    // triangle rasterization
    pub fn draw_triangle(&mut self, v0: Vec2f, v1: Vec2f, v2: Vec2f,
                         z0: f32, z1: f32, z2: f32, color: u32) {
        // TODO -> Triangle rasterization algorrthm

        // Find bounding box of triangle
        let min_x = (v0.x.min(v1.x).min(v2.x)).floor() as i32;
        let max_x = (v0.x.max(v1.x).max(v2.x)).ceil() as i32;
        let min_y = (v0.y.min(v1.y).min(v2.y).floor() as i32);
        let max_y = (v0.y.max(v1.y).max(v2.y).ceil() as i32);

        // We check for every pixel in bounding box
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let p = Vec2f::new(x as f32, y as f32);
                let (u, v, w) = self.barycentric_coordinates(p, v0, v1, v2);

                // Check if point is inside triangle
                if u >= 0.0 && v >= 0.0 && w >= 0.0 {
                    // Interpolate depth using barycentric coordinates
                    let depth = u * z0 + v * z1 + w * z2;

                    // Z buffer test and pixel drawing
                    let pixel_index = (y* self.width as i32 + x) as usize;
                    if depth < self.z_buffer[pixel_index] {
                        self.z_buffer[pixel_index] = depth;
                        self.set_pixel(x as u32, y as u32, color);
                    }
                }
            }
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
    pub fn render_frame_square(&mut self) {
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

    pub fn render_frame(&mut self) {
        self.clear(0xFF000000);

        // Create a camera
        let camera = Mat4x4::look_at(
            Vec3f::new(0.0, 2.0, 5.0),
            Vec3f::new(0.0, 0.0, 0.0),
            Vec3f::new(0.0, 1.0, 0.0)
        );

        // Same cube vertices...
        let cube_vertices = [
            Vec3f::new(-1.0, -1.0,  1.0), // 0
            Vec3f::new( 1.0, -1.0,  1.0), // 1
            Vec3f::new( 1.0,  1.0,  1.0), // 2
            Vec3f::new(-1.0,  1.0,  1.0), // 3
            Vec3f::new(-1.0, -1.0, -1.0), // 4
            Vec3f::new( 1.0, -1.0, -1.0), // 5
            Vec3f::new( 1.0,  1.0, -1.0), // 6
            Vec3f::new(-1.0,  1.0, -1.0), // 7
        ];

        // Project vertices and store 3D positions for depth
        let mut screen_vertices = Vec::new();
        let mut transformed_vertices = Vec::new();

        for vertex in &cube_vertices {
            let camera_point = camera.inverse().unwrap().multiply_point(vertex);
            if let Some(screen_pos) = self.project_to_screen(vertex, &camera) {
                screen_vertices.push(screen_pos);
                transformed_vertices.push(camera_point);
            } else {
                return;
            }
        }

        // Define triangle faces (2 triangles per face)
        let faces = [
            // Front face
            (0, 1, 2), (2, 3, 0),
            // Back face
            (4, 7, 6), (6, 5, 4),
            // Left face
            (4, 0, 3), (3, 7, 4),
            // Right face
            (1, 5, 6), (6, 2, 1),
            // Top face
            (3, 2, 6), (6, 7, 3),
            // Bottom face
            (4, 5, 1), (1, 0, 4),
        ];

        // Draw triangles instead of lines
        for (i0, i1, i2) in &faces {
            let v0 = screen_vertices[*i0];
            let v1 = screen_vertices[*i1];
            let v2 = screen_vertices[*i2];

            let z0 = transformed_vertices[*i0].z;
            let z1 = transformed_vertices[*i1].z;
            let z2 = transformed_vertices[*i2].z;

            self.draw_triangle(v0, v1, v2, z0, z1, z2, 0xFF00FF00);
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

        for depth in &mut self.z_buffer {
            *depth = f32::INFINITY;
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: u32) {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize;
            self.framebuffer[index] = color;
        }
    }
}
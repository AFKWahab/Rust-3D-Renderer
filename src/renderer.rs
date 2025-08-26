use crate::math::Vec2f;

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

    /// Given triangle with vertices A, B, C and point P, we want to find weights (u,v,w) such that
    /// P=u*A + v*B + w*C
    /// u + v + w = 1
    fn barycentric_coordinates(&self, p: Vec2f, v0: Vec2f, v1: Vec2f, v2: Vec2f) -> (f32, f32, f32) {
        let v0v1 = Vec2f::new(v1.x - v0.x, v1.y - v0.y);
        let v0v2 = Vec2f::new(v2.x - v0.x, v2.y - v0.y);
        let v0p = Vec2f::new(p.x - v0.x, p.y - v0.y);

        let dot00 = v0v2.dot(&v0v2); // v0v2 · v0v2
        let dot01 = v0v2.dot(&v0v1); // v0v2 · v0v1
        let dot02 = v0v2.dot(&v0p); // v0v2 · v0p
        let dot11 = v0v1.dot(&v0v1); // v0v1 · v0v1
        let dot12 = v0v1.dot(&v0p); // v0v1 · v0p

        // Calculate barycentric coordinates
        let inv_denom = 1.0 / (dot00 * dot11 - dot01 * dot01);
        let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
        let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;
        let w = 1.0 - u - v;

        (u, v, w)
    }

    /// Core triangle rasterization function
    pub fn draw_triangle(&mut self, v0: Vec2f, v1: Vec2f, v2: Vec2f,
                         z0: f32, z1: f32, z2: f32, color: u32) {
        // Find bounding box of triangle
        let min_x = (v0.x.min(v1.x).min(v2.x)).floor() as i32;
        let max_x = (v0.x.max(v1.x).max(v2.x)).ceil() as i32;
        let min_y = (v0.y.min(v1.y).min(v2.y)).floor() as i32;
        let max_y = (v0.y.max(v1.y).max(v2.y)).ceil() as i32;

        // Check every pixel in bounding box
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let p = Vec2f::new(x as f32, y as f32);
                let (u, v, w) = self.barycentric_coordinates(p, v0, v1, v2);

                // Check if point is inside triangle
                if u >= 0.0 && v >= 0.0 && w >= 0.0 {
                    // Interpolate depth using barycentric coordinates
                    let depth = u * z0 + v * z1 + w * z2;
                    // Z-buffer test and pixel drawing
                    let pixel_index = (y * self.width as i32 + x) as usize;
                    if pixel_index < self.z_buffer.len() && depth < self.z_buffer[pixel_index] {
                        self.z_buffer[pixel_index] = depth;
                        self.set_pixel(x as u32, y as u32, color);
                    }
                }
            }
        }
    }

    /// Bresenham's line algorithm (for debugging wireframes)
    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: u32) {
        let dx = (x1 - x0).abs();
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
        // Clear z-buffer too
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
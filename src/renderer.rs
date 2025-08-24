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

    // We do &mut self, because wew ant to borrow the Renderer, but we also modify the framebuffer, so we need mutable access
    pub fn render_frame(&mut self) {
        for pixel in &mut self.framebuffer {
            // We do * in front of pixel because pixel is a reference to &mut u32, which points to a location in memory
            // *pixel is derefencing - it accesses the actual value at that location, we need to dereference to assign a new value
            *pixel = 0xFF0000FF;
        }


        self.clear(0xFF000000);

        // Draw simple lines
        self.draw_line(100, 100, 200, 100, 0xFF00FF00);
        self.draw_line(200, 100, 150, 200, 0xFF00FF00);
        self.draw_line(150, 200, 100, 100, 0xFF00FF00);
        // TODO -> This is where the triangle rasterization will go
    }

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
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

        // TODO -> This is where the triangle rasterization will go

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
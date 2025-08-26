use crate::math::Vec3f;

#[derive(Copy, Clone)]
pub struct Triangle {
    pub indices: [usize; 3],  // Indices into vertex array
    pub color: u32,
}

impl Triangle {
    pub fn new(i0: usize, i1: usize, i2: usize, color: u32) -> Self {
        Self {
            indices: [i0, i1, i2],
            color,
        }
    }

    pub fn get_vertices(&self, mesh: &Mesh) -> (Vec3f, Vec3f, Vec3f) {
        (
            mesh.vertices[self.indices[0]],
            mesh.vertices[self.indices[1]],
            mesh.vertices[self.indices[2]],
        )
    }

    pub fn calculate_normal(&self, mesh: &Mesh) -> Vec3f {
        let (v0, v1, v2) = self.get_vertices(mesh);
        Vec3f::calculate_triangle_normal(v0, v1, v2)
    }
}

pub struct Mesh {
    pub vertices: Vec<Vec3f>,
    pub triangles: Vec<Triangle>,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            triangles: Vec::new(),
        }
    }

    pub fn add_vertex(&mut self, vertex: Vec3f) -> usize {
        self.vertices.push(vertex);
        self.vertices.len() - 1
    }

    pub fn add_triangle(&mut self, triangle: Triangle) {
        self.triangles.push(triangle);
    }

    pub fn create_cube() -> Self {
        let mut mesh = Self::new();

        // Add vertices
        let vertices = [
            Vec3f::new(-1.0, -1.0,  1.0), // 0: bottom-left-front
            Vec3f::new( 1.0, -1.0,  1.0), // 1: bottom-right-front
            Vec3f::new( 1.0,  1.0,  1.0), // 2: top-right-front
            Vec3f::new(-1.0,  1.0,  1.0), // 3: top-left-front
            Vec3f::new(-1.0, -1.0, -1.0), // 4: bottom-left-back
            Vec3f::new( 1.0, -1.0, -1.0), // 5: bottom-right-back
            Vec3f::new( 1.0,  1.0, -1.0), // 6: top-right-back
            Vec3f::new(-1.0,  1.0, -1.0), // 7: top-left-back
        ];

        for vertex in vertices {
            mesh.add_vertex(vertex);
        }

        // Add triangles with different colors for each face
        let triangles = [
            // Front face (green)
            Triangle::new(0, 1, 2, 0x00FF00),
            Triangle::new(2, 3, 0, 0x00FF00),

            // Back face (red)
            Triangle::new(4, 7, 6, 0xFF0000),
            Triangle::new(6, 5, 4, 0xFF0000),

            // Left face (blue)
            Triangle::new(4, 0, 3, 0x0000FF),
            Triangle::new(3, 7, 4, 0x0000FF),

            // Right face (yellow)
            Triangle::new(1, 5, 6, 0xFFFF00),
            Triangle::new(6, 2, 1, 0xFFFF00),

            // Top face (magenta)
            Triangle::new(3, 2, 6, 0xFF00FF),
            Triangle::new(6, 7, 3, 0xFF00FF),

            // Bottom face (cyan)
            Triangle::new(4, 5, 1, 0x00FFFF),
            Triangle::new(1, 0, 4, 0x00FFFF),
        ];

        for triangle in triangles {
            mesh.add_triangle(triangle);
        }

        mesh
    }

    pub fn create_triangle() -> Self {
        let mut mesh = Self::new();

        // Simple triangle for testing
        mesh.add_vertex(Vec3f::new(0.0, 1.0, 0.0));   // Top
        mesh.add_vertex(Vec3f::new(-1.0, -1.0, 0.0)); // Bottom left
        mesh.add_vertex(Vec3f::new(1.0, -1.0, 0.0));  // Bottom right

        mesh.add_triangle(Triangle::new(0, 1, 2, 0xFF0000));

        mesh
    }

    pub fn get_bounds(&self) -> (Vec3f, Vec3f) {
        if self.vertices.is_empty() {
            return (Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(0.0, 0.0, 0.0));
        }

        let mut min = self.vertices[0];
        let mut max = self.vertices[0];

        for vertex in &self.vertices {
            min.x = min.x.min(vertex.x);
            min.y = min.y.min(vertex.y);
            min.z = min.z.min(vertex.z);

            max.x = max.x.max(vertex.x);
            max.y = max.y.max(vertex.y);
            max.z = max.z.max(vertex.z);
        }

        (min, max)
    }

    pub fn transform_vertices(&self, transform_matrix: &crate::math::Mat4x4) -> Vec<Vec3f> {
        self.vertices
            .iter()
            .map(|vertex| transform_matrix.multiply_point(vertex))
            .collect()
    }
}
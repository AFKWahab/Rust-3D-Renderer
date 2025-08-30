use crate::math::Vec3f;

#[derive(Copy, Clone)]
pub struct Triangle {
    pub indices: [usize; 3],  // Indices into vertex array
    pub color: u32,
    pub material_id: Option<usize>, // Index into materials array
}

impl Triangle {
    pub fn new(i0: usize, i1: usize, i2: usize, color: u32) -> Self {
        Self {
            indices: [i0, i1, i2],
            color,
            material_id: None,
        }
    }

    pub fn with_material(i0: usize, i1: usize, i2: usize, color: u32, material_id: usize) -> Self {
        Self {
            indices: [i0, i1, i2],
            color,
            material_id: Some(material_id),
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

    pub fn get_center(&self, mesh: &Mesh) -> Vec3f {
        let (v0, v1, v2) = self.get_vertices(mesh);
        Vec3f::new(
            (v0.x + v1.x + v2.x) / 3.0,
            (v0.y + v1.y + v2.y) / 3.0,
            (v0.z + v1.z + v2.z) / 3.0,
        )
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

        // Add vertices - proper winding for outward-facing normals
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

        // Add triangles with proper counter-clockwise winding (when viewed from outside)
        let triangles = [
            // Front face (z = 1) - normal points toward +Z
            Triangle::new(0, 1, 2, 0xFF00FF00), // Green
            Triangle::new(2, 3, 0, 0xFF00FF00),

            // Back face (z = -1) - normal points toward -Z
            Triangle::new(5, 4, 7, 0xFFFF0000), // Red
            Triangle::new(7, 6, 5, 0xFFFF0000),

            // Left face (x = -1) - normal points toward -X
            Triangle::new(4, 0, 3, 0xFF0000FF), // Blue
            Triangle::new(3, 7, 4, 0xFF0000FF),

            // Right face (x = 1) - normal points toward +X
            Triangle::new(1, 5, 6, 0xFFFFFF00), // Yellow
            Triangle::new(6, 2, 1, 0xFFFFFF00),

            // Top face (y = 1) - normal points toward +Y
            Triangle::new(3, 2, 6, 0xFFFF00FF), // Magenta
            Triangle::new(6, 7, 3, 0xFFFF00FF),

            // Bottom face (y = -1) - normal points toward -Y
            Triangle::new(4, 5, 1, 0xFF00FFFF), // Cyan
            Triangle::new(1, 0, 4, 0xFF00FFFF),
        ];

        for triangle in triangles {
            mesh.add_triangle(triangle);
        }

        mesh
    }

    pub fn create_triangle() -> Self {
        let mut mesh = Self::new();

        // Simple triangle for testing - counter-clockwise winding
        mesh.add_vertex(Vec3f::new(0.0, 1.0, 0.0));   // Top
        mesh.add_vertex(Vec3f::new(-1.0, -1.0, 0.0)); // Bottom left
        mesh.add_vertex(Vec3f::new(1.0, -1.0, 0.0));  // Bottom right

        mesh.add_triangle(Triangle::new(0, 1, 2, 0xFFFF0000));

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

    pub fn transform_normals(&self, normal_matrix: &crate::math::Mat4x4) -> Vec<Vec3f> {
        self.triangles
            .iter()
            .map(|triangle| {
                let normal = triangle.calculate_normal(self);
                normal_matrix.multiply_vector(&normal).normalize()
            })
            .collect()
    }
}
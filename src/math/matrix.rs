use crate::math::vec3::Vec3F;

pub struct Mat4x4 {
    // Store as 16 f32 values
    pub m: [f32; 16]
}

impl Mat4x4 {
    pub fn new(m: [f32; 16]) -> Mat4x4 {
        Mat4x4 { m }
    }

    pub fn identity() -> Mat4x4 {
        Mat4x4::new([
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0
        ])
    }

    pub fn get(&self, row: usize, col: usize) -> f32 {
        self.m[row * 4 + col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: f32) {
        self.m[row * 4 + col] = value;
    }

    // Get a whole row as a slice
    pub fn get_row(&self, row: usize) -> [f32; 4] {
        [
            self.m[row * 4 + 0],
            self.m[row * 4 + 1],
            self.m[row * 4 + 2],
            self.m[row * 4 + 3],
        ]
    }

    // Get a whole column
    pub fn get_col(&self, col: usize) -> [f32; 4] {
        [
            self.m[col * 4 + 0],
            self.m[col * 4 + 1],
            self.m[col * 4 + 2],
            self.m[col * 4 + 3],
        ]
    }

    pub fn multiply(&self, other: &Mat4x4) -> Mat4x4 {
        let mut result = [0.0; 16];

        for row in 0..4 {
            for col in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 {
                    // Using helper methods - much cleaner!
                    sum += self.get(row, k) * other.get(k, col);
                }
                result[row * 4 + col] = sum;
            }
        }

        Mat4x4::new(result)
    }

    pub fn inverse(&self) -> Mat4x4 {
        // Creating augmented matrix [4x8] stored as flat array
        let mut augmented = [0.0; 32]; // 4 rows × 8 cols = 32

        // Helper functions for augmented matrix
        let get_aug = |row: usize, col: usize| -> f32 {
            augmented[row * 8 + col]
        };

        let set_aug = |aug: &mut [f32; 32], row: usize, col: usize, value: f32| {
            aug[row * 8 + col] = value;
        };

        // Fill left side with original matrix
        for row in 0..4 {
            for col in 0..4 {
                set_aug(&mut augmented, row, col, self.get(row, col));
            }
        }

        // Fill right side with identity matrix
        for row in 0..4 {
            for col in 0..4 {
                let value = if row == col { 1.0 } else { 0.0 };
                set_aug(&mut augmented, row, col + 4, value);
            }
        }

        // TODO: Gaussian elimination goes here
        // For now, return identity (placeholder)
        Mat4x4::identity()
    }
}
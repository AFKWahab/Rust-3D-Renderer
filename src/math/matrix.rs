use std::mem::zeroed;
use crate::math::vec3::Vec3f;
use crate::math::vec4::Vec4f;

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

    pub fn inverse(&self) -> Option<Mat4x4> {
        // Creating augmented matrix [4x8] stored as flat array
        let mut augmented = [0.0; 32]; // 4 rows × 8 cols = 32

        // Helper functions for augmented matrix
        let get_aug = |aug: &[f32; 32], row: usize, col: usize| -> f32 {
            aug[row * 8 + col]
        };

        let set_aug = |aug: &mut [f32; 32], row: usize, col: usize, value: f32| {
            aug[row * 8 + col] = value;
        };

        // Set up [A | I]
        for row in 0..4 {
            for col in 0..4 {
                set_aug(&mut augmented, row, col, self.get(row, col));           // Left: original matrix
                set_aug(&mut augmented, row, col + 4, if row == col { 1.0 } else { 0.0 }); // Right: identity
            }
        }

        for current_col in 0..4 {
            // Find the pivot (row with the largest absolute value in current column)
            let mut pivot_row = current_col;
            let mut max_val = get_aug(&augmented, current_col, current_col).abs();

            for row in (current_col + 1)..4 {
                let val = get_aug(&augmented, row, current_col).abs();
                if val > max_val {
                    max_val = val;
                    pivot_row = row;
                }
            }

            // Check for singular matrix (pivot is zero or very close to zero)
            if max_val < 1e-10 {
                return None; // Matrix can not be inverted
            }

            // Swap rows if necessary (partial pivoting)
            if pivot_row != current_col {
                for col in 0..8 {
                    let temp = get_aug(&augmented, current_col, col);
                    let v = get_aug(&augmented, pivot_row, col);
                    set_aug(&mut augmented, current_col, col, v);
                    set_aug(&mut augmented, pivot_row, col, temp);
                }
            }

            // Scale pivot row to make diagonal element = 1
            let pivot_element = get_aug(&augmented, current_col, current_col);
            for col in 0..8 {
                let scaled_value = get_aug(&augmented, current_col, col) / pivot_element;
                set_aug(&mut augmented, current_col, col, scaled_value);
            }

            // Eliminate all other elements in current column (make them 0)
            for row in 0..4 {
                if row != current_col {
                    let factor = get_aug(&augmented, row, current_col);
                    for col in 0..8 {
                        let new_value = get_aug(&augmented, row, col) - factor * get_aug(&augmented, current_col, col);
                        set_aug(&mut augmented, row, col, new_value);
                    }
                }
            }
        }

        // Extract the inverse matrix from the right half of augmented matrix
        let mut result = [0.0; 16];
        for row in 0..4 {
            for col in 0..4 {
                result[row * 4 + col] = get_aug(&augmented, row, col + 4);
            }
        }
        Some(Mat4x4::new(result))
    }

    ///
    /// The translation matrix moves (translates) points from one position to another.
    /// What this means is that we can ask it to "move this point 5 units right, 3 units up, 2 units forward
    /// But this translation matrix in itself, don't do much, all we need it for is to initialize it, and then multiply to translate.
    /// So it can move objects, but also the camera etc.
    ///
    pub fn translation(x: f32, y: f32, z: f32) -> Mat4x4 {
        Mat4x4::new([
            1.0, 0.0, 0.0, x,
            0.0, 1.0, 0.0, y,
            0.0, 0.0, 1.0, z,
            0.0, 0.0, 0.0, 1.0,
        ])
    }


    ///
    /// X-axis rotation (pitch), this rotates in the Y Z plane.
    /// Generally the math here says that the X row stays the same
    /// The Y row: Y = Y*cos - Z*sin
    /// The Z row: Z = Y*sin + Z*cos
    /// W row is always [0.0.0.1]
    pub fn rotation_x(angle: f32) -> Mat4x4 {
        let cos_a = angle.cos();
        let sin_a = angle.sin();

        Mat4x4::new([
            1.0,   0.0,    0.0,   0.0,  // X row: X stays the same
            0.0,  cos_a, -sin_a, 0.0,  // Y row: Y = Y*cos - Z*sin
            0.0,  sin_a,  cos_a, 0.0,  // Z row: Z = Y*sin + Z*cos
            0.0,   0.0,    0.0,  1.0,  // W row: always [0,0,0,1]
        ])
    }

    ///
    /// Y-axis rotation (yaw), this rotates in the X Z plane
    /// Generally the math here says that
    /// The X row: X = X*cos + Z*sin
    /// The Z row: Z = -X*sin + Z*cos
    /// W row: always [0,0,0,1]
    pub fn rotation_y(angle: f32) -> Mat4x4 {
        let cos_a = angle.cos();
        let sin_a = angle.sin();

        Mat4x4::new([
            cos_a, 0.0,  sin_a, 0.0,  // X row: X = X*cos + Z*sin
            0.0,  1.0,   0.0,  0.0,  // Y row: Y stays the same
            -sin_a, 0.0,  cos_a, 0.0,  // Z row: Z = -X*sin + Z*cos
            0.0,  0.0,   0.0,  1.0,  // W row: always [0,0,0,1]
        ])
    }

    ///
    /// Z-axis rotation (roll), this rotates in the X Y plane
    /// Generally the math here says that
    /// The X row: X = X*cos + Y*sin
    /// The Y row: Z = X*sin + Y*cos
    /// W row: always [0,0,0,1]
    pub fn rotation_z(angle: f32) -> Mat4x4 {
        let cos_a = angle.cos();
        let sin_a = angle.sin();

        Mat4x4::new([
            cos_a, -sin_a, 0.0, 0.0,  // X row: X = X*cos - Y*sin
            sin_a,  cos_a, 0.0, 0.0,  // Y row: Y = X*sin + Y*cos
            0.0,    0.0,  1.0, 0.0,  // Z row: Z stays the same
            0.0,    0.0,  0.0, 1.0,  // W row: always [0,0,0,1]
        ])
    }

    ///
    /// The point of scaling is to multiply each coordinate by a scale factor.
    ///
    pub fn scale(x: f32, y: f32, z: f32) -> Mat4x4 {
        Mat4x4::new([
            x, 0.0, 0.0, 0.0,
            0.0, y, 0.0, 0.0,
            0.0, 0.0, z, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    ///
    /// This transforms a position in 3D space.
    /// Affected by translation (gets moved)
    /// Affected by rotation (gets rotated)
    /// Affected by scaling (gets scaled)
    /// Uses w=1 in homogenous coordinates
    /// This is used for vertex positions of a 3D model, camera position, light positions, any "where is this thing" coordinate
    ///
    pub fn multiply_point(&self, point: &Vec3f) -> Vec3f {
        // Create a Vec4f out of the Vec3f, using helper method
        let vector_4d = Vec4f::from_point(point);

        let mut result = Vec4f::new(0.0, 0.0, 0.0, 0.0);

        for row in 0..4 {
            let mut sum = 0.0;

            // Multiply row by vector: row[0]*vec.x + row[1]*vec.y + row[2]*vec.z + row[3]*vec.w
            sum += self.get(row, 0) * vector_4d.x;
            sum += self.get(row, 1) * vector_4d.y;
            sum += self.get(row, 2) * vector_4d.z;
            sum += self.get(row, 3) * vector_4d.w;

            // Store result in appropriate component
            match row {
                0 => result.x = sum,
                1 => result.y = sum,
                2 => result.z = sum,
                3 => result.w = sum,
                _ => unreachable!(),
            }
        }

        result.to_Vec3f()  // Convert back to Vec3f
    }

    ///
    /// Transforms a direction in 3D space
    /// Not affected by translation (directions don't have positions)
    /// Affected by rotation (gets rotated)
    /// Affected by scaling (gets scaled)
    /// Uses w=0 in homogeneous coordinates
    /// This is used for surface normals, light directions, velocity vectors, any "which direction" vector
    ///
    pub fn multiply_vector(&self, vector: &Vec3f) -> Vec3f {
        // Create a Vec4f out of the Vec3f, using helper method (w=0)
        let vector_4d = Vec4f::from_vector(vector);

        let mut result = Vec4f::new(0.0, 0.0, 0.0, 0.0);

        for row in 0..4 {
            let mut sum = 0.0;

            // Multiply row by vector: row[0]*vec.x + row[1]*vec.y + row[2]*vec.z + row[3]*vec.w
            sum += self.get(row, 0) * vector_4d.x;
            sum += self.get(row, 1) * vector_4d.y;
            sum += self.get(row, 2) * vector_4d.z;
            sum += self.get(row, 3) * vector_4d.w;  // This will be 0, so translation is ignored!

            // Store result in appropriate component
            match row {
                0 => result.x = sum,
                1 => result.y = sum,
                2 => result.z = sum,
                3 => result.w = sum,
                _ => unreachable!(),
            }
        }

        result.to_Vec3f()  // Convert back to Vec3f
    }

    pub fn look_at(eye: Vec3f, target: Vec3f, up: Vec3f) -> Mat4x4 {
        // Step 1: Calculate forward vector (direction camera is looking)
        let forward = (target - eye).normalize();

        // Step 2: Calculate right vector (camera's right direction)
        let right = forward.cross(&up).normalize();

        // Step 3: Calculate actual up vector (camera's up direction)
        let camera_up = right.cross(&forward);

        // Step 4: Create view matrix
        // Note: Forward is negated because camera looks down -Z axis by convention
        Mat4x4::new([
            right.x,     right.y,     right.z,     -right.dot(&eye),
            camera_up.x, camera_up.y, camera_up.z, -camera_up.dot(&eye),
            -forward.x,  -forward.y,  -forward.z,  forward.dot(&eye),
            0.0,         0.0,         0.0,         1.0,
        ])
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to check if two matrices are approximately equal
    fn matrices_approx_equal(a: &Mat4x4, b: &Mat4x4, tolerance: f32) -> bool {
        for row in 0..4 {
            for col in 0..4 {
                let diff = (a.get(row, col) - b.get(row, col)).abs();
                if diff > tolerance {
                    println!("Matrices differ at [{},{}]: {} vs {}, diff: {}",
                             row, col, a.get(row, col), b.get(row, col), diff);
                    return false;
                }
            }
        }
        true
    }

    #[test]
    fn test_identity_inverse() {
        let identity = Mat4x4::identity();
        let inverse = identity.inverse().expect("Identity should be invertible");

        assert!(matrices_approx_equal(&identity, &inverse, 1e-6),
                "Identity matrix inverse should be itself");
    }

    #[test]
    fn test_simple_matrix_inverse() {
        let matrix = Mat4x4::new([
            2.0, 0.0, 0.0, 0.0,
            0.0, 3.0, 0.0, 0.0,
            0.0, 0.0, 4.0, 0.0,
            0.0, 0.0, 0.0, 5.0,
        ]);

        let expected_inverse = Mat4x4::new([
            0.5, 0.0, 0.0, 0.0,
            0.0, 1.0 / 3.0, 0.0, 0.0,
            0.0, 0.0, 0.25, 0.0,
            0.0, 0.0, 0.0, 0.2,
        ]);

        let inverse = matrix.inverse().expect("Matrix should be invertible");
        assert!(matrices_approx_equal(&inverse, &expected_inverse, 1e-6),
                "Simple diagonal matrix inverse incorrect");
    }

    #[test]
    fn test_matrix_multiply_inverse_equals_identity() {
        let matrix = Mat4x4::new([
            1.0, 2.0, 0.0, 1.0,
            0.0, 1.0, 1.0, 2.0,
            1.0, 0.0, 1.0, 0.0,
            0.0, 1.0, 0.0, 2.0,
        ]);

        let inverse = matrix.inverse().expect("Matrix should be invertible");
        let result = matrix.multiply(&inverse);
        let identity = Mat4x4::identity();

        assert!(matrices_approx_equal(&result, &identity, 1e-5),
                "A * A^(-1) should equal identity matrix");
    }

    #[test]
    fn test_inverse_multiply_matrix_equals_identity() {
        // Test A^(-1) * A = I
        let matrix = Mat4x4::new([
            2.0, 1.0, 0.0, 0.0,
            1.0, 2.0, 1.0, 0.0,
            0.0, 1.0, 2.0, 1.0,
            0.0, 0.0, 1.0, 2.0,
        ]);

        let inverse = matrix.inverse().expect("Matrix should be invertible");
        let result = inverse.multiply(&matrix);
        let identity = Mat4x4::identity();

        assert!(matrices_approx_equal(&result, &identity, 1e-5),
                "A^(-1) * A should equal identity matrix");
    }

    #[test]
    fn test_singular_matrix() {
        // Create a singular (non-invertible) matrix
        let singular_matrix = Mat4x4::new([
            1.0, 2.0, 3.0, 4.0,
            1.0, 2.0, 3.0, 4.0,  // Same as first row
            1.0, 2.0, 3.0, 4.0,  // Same as first row
            1.0, 2.0, 3.0, 4.0,  // Same as first row
        ]);

        let result = singular_matrix.inverse();
        assert!(result.is_none(), "Singular matrix should not be invertible");
    }

    #[test]
    fn test_another_singular_matrix() {
        // Create another singular matrix - zero row
        let singular_matrix = Mat4x4::new([
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 0.0,  // Zero row makes it singular
        ]);

        let result = singular_matrix.inverse();
        assert!(result.is_none(), "Matrix with zero row should not be invertible");
    }

    #[test]
    fn test_known_matrix_with_known_inverse() {
        // Matrix:
        // [ 4, 0, 0, 0 ]
        // [ 0, 0, 2, 0 ]
        // [ 0, 1, 2, 0 ]
        // [ 1, 0, 0, 1 ]
        let matrix = Mat4x4::new([
            4.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 2.0, 0.0,
            0.0, 1.0, 2.0, 0.0,
            1.0, 0.0, 0.0, 1.0,
        ]);

        // Correct inverse:
        // [  0.25,  0.0,  0.0, 0.0 ]
        // [  0.0,  -1.0,  1.0, 0.0 ]
        // [  0.0,   0.5,  0.0, 0.0 ]
        // [ -0.25,  0.0,  0.0, 1.0 ]
        let expected_inverse = Mat4x4::new([
            0.25, 0.0, 0.0, 0.0,
            0.0, -1.0, 1.0, 0.0,
            0.0, 0.5, 0.0, 0.0,
            -0.25, 0.0, 0.0, 1.0,
        ]);

        let inverse = matrix.inverse().expect("Matrix should be invertible");
        assert!(
            matrices_approx_equal(&inverse, &expected_inverse, 1e-5),
            "Known matrix inverse should match expected result"
        );
    }
}
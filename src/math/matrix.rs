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
}


// Add this to the bottom of your matrix.rs file

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
        // Test with a simple 4x4 matrix we can verify by hand
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
        // Use a non‑singular matrix (det ≠ 0).
        // This is your original matrix but with the last element changed from 1.0 → 2.0 (det = 3).
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
        // Test A^(-1) * A = I (should also work)
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
        // Create a singular (non-invertible) matrix - all rows are the same
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
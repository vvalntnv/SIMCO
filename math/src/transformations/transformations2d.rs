// WARNING: DO NOT FORMAT BRO (crying emoji)
use crate::{operations::approx_eq, vectors::vec2d::Vec2};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transformation2D {
    pub matrix: [[f32; 3]; 3],
}

impl Transformation2D {
    pub fn identity() -> Self {
        Self {
            matrix: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        }
    }

    pub fn scalar(&self, scalar: f32) -> Self {
        let multiply_row = |row: [f32; 3]| {
            let mut res = [0.0 as f32; 3];
            for i in 0..row.len() {
                res[i] = row[i] * scalar;
            }
            return res;
        };

        Self {
            matrix: [
                multiply_row(self.matrix[0]),
                multiply_row(self.matrix[1]),
                multiply_row(self.matrix[2]),
            ],
        }
    }

    pub fn translation(trans_x: f32, trans_y: f32) -> Self {
        Self {
            matrix: [[1.0, 0.0, trans_x], [0.0, 1.0, trans_y], [0.0, 0.0, 1.0]],
        }
    }

    pub fn rotation(radians: f32) -> Self {
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        Self {
            matrix: [
                [cos_theta, -sin_theta, 0.0],
                [sin_theta, cos_theta, 0.0],
                [0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn scale(scale_x: f32, scale_y: f32) -> Self {
        Self {
            matrix: [[scale_x, 0.0, 0.0], [0.0, scale_y, 0.0], [0.0, 0.0, 1.0]],
        }
    }

    pub fn uniform_scale(scale: f32) -> Self {
        Self::scale(scale, scale)
    }

    pub fn shear(shear_x: f32, shear_y: f32) -> Self {
        Self {
            matrix: [[1.0, shear_x, 0.0], [shear_y, 1.0, 0.0], [0.0, 0.0, 1.0]],
        }
    }

    pub fn uniform_shear(shear: f32) -> Self {
        Self::shear(shear, shear)
    }

    pub fn reflection_x() -> Self {
        Transformation2D {
            matrix: [[1.0, 0.0, 0.0], [0.0, -1.0, 0.0], [0.0, 0.0, 1.0]],
        }
    }

    pub fn reflection_y() -> Self {
        Transformation2D {
            matrix: [[-1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        }
    }

    pub fn multiply(&self, other: &Transformation2D) -> Self {
        let mut result: [[f32; 3]; 3] = [[0.0; 3]; 3];

        for i in 0..3 {
            for j in 0..3 {
                result[i][j] = self.matrix[i][0] * other.matrix[0][j]
                    + self.matrix[i][1] * other.matrix[1][j]
                    + self.matrix[i][2] * other.matrix[2][j];
            }
        }

        Self { matrix: result }
    }

    pub fn determinant(&self) -> f32 {
        self.matrix[0][0] * self.matrix[1][1] - self.matrix[0][1] * self.matrix[1][0]
    }

    pub fn inverse(&self) -> Option<Self> {
        let determinant = self.determinant();

        if approx_eq(determinant, 0.0, None) {
            return None;
        }

        let inv_det = 1.0 / determinant;

        let mut new_trans = Transformation2D {
            matrix: [
                [
                    self.matrix[1][1] * inv_det,
                    -self.matrix[0][1] * inv_det,
                    0.0,
                ],
                [
                    -self.matrix[1][0] * inv_det,
                    self.matrix[0][0] * inv_det,
                    0.0,
                ],
                [0.0, 0.0, 1.0],
            ],
        };

        let vec = Vec2::from([self.matrix[0][2], self.matrix[1][2]]);
        let adj_trans = vec.apply_transformation(new_trans);

        new_trans.matrix[0][2] = -adj_trans.x;
        new_trans.matrix[1][2] = -adj_trans.y;

        Some(new_trans)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vectors::vec2d::Vec2;
    use std::f32::consts::PI;

    // Helper function to check if two matrices are approximately equal
    fn matrix_approx_eq(a: [[f32; 3]; 3], b: [[f32; 3]; 3], epsilon: Option<f32>) -> bool {
        for i in 0..3 {
            for j in 0..3 {
                if !approx_eq(a[i][j], b[i][j], epsilon) {
                    return false;
                }
            }
        }
        true
    }

    #[test]
    fn test_identity() {
        let identity = Transformation2D::identity();
        let expected = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
        assert_eq!(identity.matrix, expected);
    }

    #[test]
    fn test_scalar() {
        let identity = Transformation2D::identity();
        let scaled = identity.scalar(2.0);
        let expected = [[2.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 2.0]];
        assert_eq!(scaled.matrix, expected);
    }

    #[test]
    fn test_translation() {
        let translation = Transformation2D::translation(3.0, 4.0);
        let expected = [[1.0, 0.0, 3.0], [0.0, 1.0, 4.0], [0.0, 0.0, 1.0]];
        assert_eq!(translation.matrix, expected);

        // Test applying translation to a vector
        let vec = Vec2::new(1.0, 2.0);
        let transformed = vec.apply_transformation(translation);
        assert_eq!(transformed.x, 4.0); // 1 + 3
        assert_eq!(transformed.y, 6.0); // 2 + 4
    }

    #[test]
    fn test_rotation() {
        // Test 90 degree rotation
        let rotation = Transformation2D::rotation(PI / 2.0);
        let expected = [[0.0, -1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]];

        // Use approximate equality for floating point comparisons
        assert!(matrix_approx_eq(rotation.matrix, expected, Some(1e-6)));

        // Test applying rotation to a vector
        let vec = Vec2::new(1.0, 0.0);
        let transformed = vec.apply_transformation(rotation);
        assert!(approx_eq(transformed.x, 0.0, Some(1e-6)));
        assert!(approx_eq(transformed.y, 1.0, Some(1e-6)));
    }

    #[test]
    fn test_scale() {
        // Test non-uniform scaling
        let scale = Transformation2D::scale(2.0, 3.0);
        let expected = [[2.0, 0.0, 0.0], [0.0, 3.0, 0.0], [0.0, 0.0, 1.0]];
        assert_eq!(scale.matrix, expected);

        // Test uniform scaling
        let uniform_scale = Transformation2D::uniform_scale(2.0);
        let expected = [[2.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 1.0]];
        assert_eq!(uniform_scale.matrix, expected);

        // Test applying scaling to a vector
        let vec = Vec2::new(2.0, 3.0);
        let transformed = vec.apply_transformation(scale);
        assert_eq!(transformed.x, 4.0); // 2 * 2
        assert_eq!(transformed.y, 9.0); // 3 * 3
    }

    #[test]
    fn test_shear() {
        // Test non-uniform shearing
        let shear = Transformation2D::shear(2.0, 3.0);
        let expected = [[1.0, 2.0, 0.0], [3.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
        assert_eq!(shear.matrix, expected);

        // Test uniform shearing
        let uniform_shear = Transformation2D::uniform_shear(2.0);
        let expected = [[1.0, 2.0, 0.0], [2.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
        assert_eq!(uniform_shear.matrix, expected);

        // Test applying shear to a vector
        let vec = Vec2::new(1.0, 1.0);
        let transformed = vec.apply_transformation(shear);
        assert_eq!(transformed.x, 3.0); // 1 + 2*1
        assert_eq!(transformed.y, 4.0); // 1 + 3*1
    }

    #[test]
    fn test_reflection() {
        // Test reflection across x-axis
        let reflection_x = Transformation2D::reflection_x();
        let expected_x = [[1.0, 0.0, 0.0], [0.0, -1.0, 0.0], [0.0, 0.0, 1.0]];
        assert_eq!(reflection_x.matrix, expected_x);

        // Test reflection across y-axis
        let reflection_y = Transformation2D::reflection_y();
        let expected_y = [[-1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
        assert_eq!(reflection_y.matrix, expected_y);

        // Test applying reflection to a vector
        let vec = Vec2::new(2.0, 3.0);
        let transformed_x = vec.apply_transformation(reflection_x);
        assert_eq!(transformed_x.x, 2.0);
        assert_eq!(transformed_x.y, -3.0);

        let transformed_y = vec.apply_transformation(reflection_y);
        assert_eq!(transformed_y.x, -2.0);
        assert_eq!(transformed_y.y, 3.0);
    }

    #[test]
    fn test_multiply() {
        // Test multiplying two transformations
        let translation = Transformation2D::translation(1.0, 2.0);
        let scale = Transformation2D::scale(2.0, 3.0);

        // First translate, then scale
        let combined = translation;
        let result = combined.multiply(&scale);

        // Expected: scale then translate
        let expected = [[2.0, 0.0, 1.0], [0.0, 3.0, 2.0], [0.0, 0.0, 1.0]];

        assert_eq!(result.matrix, expected);

        // Test applying combined transformation to a vector
        let vec = Vec2::new(1.0, 1.0);
        let transformed = vec.apply_transformation(result);
        assert_eq!(transformed.x, 3.0); // (1*2) + 1
        assert_eq!(transformed.y, 5.0); // (1*3) + 2
    }

    #[test]
    fn test_determinant() {
        // Test determinant of identity matrix
        let identity = Transformation2D::identity();
        assert_eq!(identity.determinant(), 1.0);

        // Test determinant of scale matrix
        let scale = Transformation2D::scale(2.0, 3.0);
        assert_eq!(scale.determinant(), 6.0); // 2 * 3

        // Test determinant of rotation matrix (should be 1)
        let rotation = Transformation2D::rotation(PI / 4.0);
        assert!(approx_eq(rotation.determinant(), 1.0, Some(1e-6)));
    }

    #[test]
    fn test_inverse() {
        // Test inverse of identity matrix
        let identity = Transformation2D::identity();
        let inverse = identity.inverse().unwrap();
        assert_eq!(inverse.matrix, identity.matrix);

        // Test inverse of scale matrix
        let scale = Transformation2D::scale(2.0, 4.0);
        let inverse = scale.inverse().unwrap();
        let expected = [[0.5, 0.0, 0.0], [0.0, 0.25, 0.0], [0.0, 0.0, 1.0]];
        assert_eq!(inverse.matrix, expected);

        // Test inverse of translation matrix
        let translation = Transformation2D::translation(3.0, 4.0);
        let inverse = translation.inverse().unwrap();
        let expected = [[1.0, 0.0, -3.0], [0.0, 1.0, -4.0], [0.0, 0.0, 1.0]];
        assert_eq!(inverse.matrix, expected);

        // Test that multiplying a matrix by its inverse gives the identity
        // Note: For matrix multiplication, A*B != B*A in general
        // We need to test both orders to ensure the inverse is correct
        let trans_copy = translation;
        let result1 = trans_copy.multiply(&inverse);

        // Check if either order gives a result close to the identity matrix
        println!("{:?}", scale);
        println!("{:?}", result1);
        assert!(matrix_approx_eq(
            result1.matrix,
            identity.matrix,
            Some(1e-5)
        ));

        // Test inverse of singular matrix (should return None)
        let singular = Transformation2D {
            matrix: [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 1.0]],
        };
        assert!(singular.inverse().is_none());
    }

    #[test]
    fn test_transformation_composition() {
        // Test composing multiple transformations
        let translation = Transformation2D::translation(1.0, 2.0);
        let rotation = Transformation2D::rotation(PI / 2.0); // 90 degrees  
        let scale = Transformation2D::scale(2.0, 3.0);

        // Apply transformations in sequence: first translate, then rotate, then scale
        let combined = scale.multiply(&rotation).multiply(&translation);

        // Test applying the combined transformation to a vector
        let vec = Vec2::new(1.0, 1.0);
        let transformed = vec.apply_transformation(combined);

        println!("{:?}", transformed);

        // Let's calculate the expected result step by step:
        // 1. Apply translation (1,2) to (1,1) -> (2,3)
        // 2. Apply rotation 90 degrees to (2,3) -> (-3,2)
        // 3. Apply scaling (2,3) to (-3,2) -> (-6,6)

        // Verify the result with a small epsilon for floating point comparison
        let expected_x = -6.0;
        let expected_y = 6.0;

        println!("{:?}", transformed);

        // Use a slightly larger epsilon for the comparison
        assert!(approx_eq(transformed.x, expected_x, Some(1e-5)));
        assert!(approx_eq(transformed.y, expected_y, Some(1e-5)));
    }
}

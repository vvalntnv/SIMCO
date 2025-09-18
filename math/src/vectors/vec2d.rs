use crate::transformations::transformations2d::Transformation2D;

#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn sub(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn scale(self, s: f32) -> Vec2 {
        Vec2 {
            x: self.x * s,
            y: self.y * s,
        }
    }

    pub fn magnitude(self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn dot_prod(self, other: Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }

    // proj_onto_vector-self
    pub fn project_to(self, onto_vector: Vec2) -> Vec2 {
        let dot_product = self.dot_prod(onto_vector);
        let magnitude = onto_vector.magnitude().abs();
        let c = dot_product / magnitude.powi(2);

        onto_vector.scale(c)
    }

    pub fn zero() -> Vec2 {
        Vec2 { x: 0.0, y: 0.0 }
    }

    pub fn apply_transformation(&self, transformation: Transformation2D) -> Vec2 {
        // default transformation + displacement/translation
        let t = transformation.matrix;
        Vec2 {
            x: t[0][0] * self.x + t[0][1] * self.y + t[0][2],
            y: t[1][0] * self.x + t[1][1] * self.y + t[1][2],
        }
    }
}

impl From<[f32; 2]> for Vec2 {
    fn from(value: [f32; 2]) -> Self {
        Vec2 {
            x: value[0],
            y: value[1],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::operations::approx_eq;
    use crate::transformations::transformations2d::Transformation2D;
    use std::f32::consts::PI;

    // Helper function to check if two Vec2 are approximately equal
    fn vec2_approx_eq(a: Vec2, b: Vec2, epsilon: Option<f32>) -> bool {
        approx_eq(a.x, b.x, epsilon) && approx_eq(a.y, b.y, epsilon)
    }

    #[test]
    fn test_new() {
        let vec = Vec2::new(3.0, 4.0);
        assert_eq!(vec.x, 3.0);
        assert_eq!(vec.y, 4.0);

        // Test with negative values
        let vec_neg = Vec2::new(-2.5, -1.7);
        assert_eq!(vec_neg.x, -2.5);
        assert_eq!(vec_neg.y, -1.7);

        // Test with zero values
        let vec_zero = Vec2::new(0.0, 0.0);
        assert_eq!(vec_zero.x, 0.0);
        assert_eq!(vec_zero.y, 0.0);

        // Test with large values
        let vec_large = Vec2::new(f32::MAX, f32::MAX);
        assert_eq!(vec_large.x, f32::MAX);
        assert_eq!(vec_large.y, f32::MAX);

        // Test with small values
        let vec_small = Vec2::new(f32::MIN_POSITIVE, f32::MIN_POSITIVE);
        assert_eq!(vec_small.x, f32::MIN_POSITIVE);
        assert_eq!(vec_small.y, f32::MIN_POSITIVE);
    }

    #[test]
    fn test_zero() {
        let zero = Vec2::zero();
        assert_eq!(zero.x, 0.0);
        assert_eq!(zero.y, 0.0);
    }

    #[test]
    fn test_add() {
        let vec1 = Vec2::new(1.0, 2.0);
        let vec2 = Vec2::new(3.0, 4.0);
        let result = vec1.add(vec2);

        assert_eq!(result.x, 4.0);
        assert_eq!(result.y, 6.0);

        // Test adding zero vector
        let zero = Vec2::zero();
        let result_zero = vec1.add(zero);
        assert_eq!(result_zero.x, vec1.x);
        assert_eq!(result_zero.y, vec1.y);

        // Test adding negative vectors
        let vec_neg = Vec2::new(-1.0, -2.0);
        let result_neg = vec1.add(vec_neg);
        assert_eq!(result_neg.x, 0.0);
        assert_eq!(result_neg.y, 0.0);

        // Test with floating point precision
        let vec3 = Vec2::new(0.1, 0.2);
        let vec4 = Vec2::new(0.3, 0.4);
        let result_float = vec3.add(vec4);
        assert!(approx_eq(result_float.x, 0.4, Some(1e-6)));
        assert!(approx_eq(result_float.y, 0.6, Some(1e-6)));

        // Test commutativity: a + b = b + a
        let a = Vec2::new(2.5, -1.8);
        let b = Vec2::new(-0.3, 4.2);
        assert!(vec2_approx_eq(a.add(b), b.add(a), Some(1e-6)));
    }

    #[test]
    fn test_sub() {
        let vec1 = Vec2::new(5.0, 7.0);
        let vec2 = Vec2::new(2.0, 3.0);
        let result = vec1.sub(vec2);

        assert_eq!(result.x, 3.0);
        assert_eq!(result.y, 4.0);

        // Test subtracting zero vector
        let zero = Vec2::zero();
        let result_zero = vec1.sub(zero);
        assert_eq!(result_zero.x, vec1.x);
        assert_eq!(result_zero.y, vec1.y);

        // Test subtracting from zero vector
        let result_from_zero = zero.sub(vec1);
        assert_eq!(result_from_zero.x, -vec1.x);
        assert_eq!(result_from_zero.y, -vec1.y);

        // Test subtracting itself (should give zero)
        let result_self = vec1.sub(vec1);
        assert_eq!(result_self.x, 0.0);
        assert_eq!(result_self.y, 0.0);

        // Test with negative results
        let vec3 = Vec2::new(1.0, 2.0);
        let vec4 = Vec2::new(3.0, 5.0);
        let result_neg = vec3.sub(vec4);
        assert_eq!(result_neg.x, -2.0);
        assert_eq!(result_neg.y, -3.0);

        // Test floating point precision
        let vec5 = Vec2::new(0.7, 0.9);
        let vec6 = Vec2::new(0.3, 0.4);
        let result_float = vec5.sub(vec6);
        assert!(approx_eq(result_float.x, 0.4, Some(1e-6)));
        assert!(approx_eq(result_float.y, 0.5, Some(1e-6)));
    }

    #[test]
    fn test_scale() {
        let vec = Vec2::new(3.0, 4.0);

        // Test positive scaling
        let scaled_pos = vec.scale(2.0);
        assert_eq!(scaled_pos.x, 6.0);
        assert_eq!(scaled_pos.y, 8.0);

        // Test negative scaling (reverses direction)
        let scaled_neg = vec.scale(-1.0);
        assert_eq!(scaled_neg.x, -3.0);
        assert_eq!(scaled_neg.y, -4.0);

        // Test zero scaling
        let scaled_zero = vec.scale(0.0);
        assert_eq!(scaled_zero.x, 0.0);
        assert_eq!(scaled_zero.y, 0.0);

        // Test scaling by 1 (identity)
        let scaled_one = vec.scale(1.0);
        assert_eq!(scaled_one.x, vec.x);
        assert_eq!(scaled_one.y, vec.y);

        // Test fractional scaling
        let scaled_frac = vec.scale(0.5);
        assert_eq!(scaled_frac.x, 1.5);
        assert_eq!(scaled_frac.y, 2.0);

        // Test with floating point precision
        let vec_float = Vec2::new(1.0, 1.0);
        let scaled_float = vec_float.scale(1.0 / 3.0);
        assert!(approx_eq(scaled_float.x, 1.0 / 3.0, Some(1e-6)));
        assert!(approx_eq(scaled_float.y, 1.0 / 3.0, Some(1e-6)));

        // Test scaling zero vector
        let zero = Vec2::zero();
        let scaled_zero_vec = zero.scale(5.0);
        assert_eq!(scaled_zero_vec.x, 0.0);
        assert_eq!(scaled_zero_vec.y, 0.0);
    }

    #[test]
    fn test_magnitude() {
        // Test basic magnitude calculation
        let vec = Vec2::new(3.0, 4.0);
        let mag = vec.magnitude();
        assert_eq!(mag, 5.0); // 3-4-5 triangle

        // Test zero vector magnitude
        let zero = Vec2::zero();
        assert_eq!(zero.magnitude(), 0.0);

        // Test unit vector magnitude
        let unit_x = Vec2::new(1.0, 0.0);
        let unit_y = Vec2::new(0.0, 1.0);
        assert_eq!(unit_x.magnitude(), 1.0);
        assert_eq!(unit_y.magnitude(), 1.0);

        // Test negative components (magnitude should be positive)
        let vec_neg = Vec2::new(-3.0, -4.0);
        assert_eq!(vec_neg.magnitude(), 5.0);

        // Test mixed signs
        let vec_mixed = Vec2::new(-3.0, 4.0);
        assert_eq!(vec_mixed.magnitude(), 5.0);

        // Test with floating point precision
        let vec_float = Vec2::new(1.0, 1.0);
        let expected_mag = (2.0_f32).sqrt();
        assert!(approx_eq(vec_float.magnitude(), expected_mag, Some(1e-6)));

        // Test very small vector
        let vec_small = Vec2::new(1e-10, 1e-10);
        let expected_small = (2e-20_f32).sqrt();
        assert!(approx_eq(
            vec_small.magnitude(),
            expected_small,
            Some(1e-20)
        ));

        // Test large vector
        let vec_large = Vec2::new(1000.0, 1000.0);
        let expected_large = 1000.0 * (2.0_f32).sqrt();
        assert!(approx_eq(vec_large.magnitude(), expected_large, Some(1e-3)));
    }

    #[test]
    fn test_dot_prod() {
        // Test basic dot product
        let vec1 = Vec2::new(2.0, 3.0);
        let vec2 = Vec2::new(4.0, 5.0);
        let dot = vec1.dot_prod(vec2);
        assert_eq!(dot, 23.0); // 2*4 + 3*5 = 8 + 15 = 23

        // Test dot product with zero vector
        let zero = Vec2::zero();
        assert_eq!(vec1.dot_prod(zero), 0.0);
        assert_eq!(zero.dot_prod(vec1), 0.0);

        // Test dot product of vector with itself (squared magnitude)
        let vec = Vec2::new(3.0, 4.0);
        let self_dot = vec.dot_prod(vec);
        assert_eq!(self_dot, 25.0); // 3² + 4² = 9 + 16 = 25
        assert_eq!(self_dot, vec.magnitude().powi(2));

        // Test orthogonal vectors (dot product should be 0)
        let vec_x = Vec2::new(1.0, 0.0);
        let vec_y = Vec2::new(0.0, 1.0);
        assert_eq!(vec_x.dot_prod(vec_y), 0.0);

        // Test parallel vectors (same direction)
        let vec3 = Vec2::new(1.0, 2.0);
        let vec4 = Vec2::new(2.0, 4.0); // 2 * vec3
        let parallel_dot = vec3.dot_prod(vec4);
        assert_eq!(parallel_dot, 10.0); // 1*2 + 2*4 = 2 + 8 = 10

        // Test antiparallel vectors (opposite direction)
        let vec5 = Vec2::new(1.0, 2.0);
        let vec6 = Vec2::new(-2.0, -4.0); // -2 * vec5
        let antiparallel_dot = vec5.dot_prod(vec6);
        assert_eq!(antiparallel_dot, -10.0); // 1*(-2) + 2*(-4) = -2 + (-8) = -10

        // Test commutativity: a · b = b · a
        let a = Vec2::new(2.5, -1.8);
        let b = Vec2::new(-0.3, 4.2);
        assert_eq!(a.dot_prod(b), b.dot_prod(a));

        // Test with floating point precision
        let vec7 = Vec2::new(0.1, 0.2);
        let vec8 = Vec2::new(0.3, 0.4);
        let dot_float = vec7.dot_prod(vec8);
        let expected = 0.1 * 0.3 + 0.2 * 0.4; // 0.03 + 0.08 = 0.11
        assert!(approx_eq(dot_float, expected, Some(1e-6)));
    }

    #[test]
    fn test_project_to() {
        // Test projection onto x-axis
        let vec = Vec2::new(3.0, 4.0);
        let x_axis = Vec2::new(1.0, 0.0);
        let proj_x = vec.project_to(x_axis);
        assert_eq!(proj_x.x, 3.0);
        assert_eq!(proj_x.y, 0.0);

        // Test projection onto y-axis
        let y_axis = Vec2::new(0.0, 1.0);
        let proj_y = vec.project_to(y_axis);
        assert_eq!(proj_y.x, 0.0);
        assert_eq!(proj_y.y, 4.0);

        // Test projection of vector onto itself (should equal the original vector)
        let proj_self = vec.project_to(vec);
        assert!(vec2_approx_eq(proj_self, vec, Some(1e-6)));

        // Test projection onto parallel vector
        let vec2 = Vec2::new(6.0, 8.0); // 2 * vec
        let proj_parallel = vec.project_to(vec2);
        assert!(vec2_approx_eq(proj_parallel, vec, Some(1e-6)));

        // Test projection onto orthogonal vector (should be zero)
        let orthogonal = Vec2::new(-4.0, 3.0); // perpendicular to (3,4)
        let proj_ortho = vec.project_to(orthogonal);
        assert!(vec2_approx_eq(proj_ortho, Vec2::zero(), Some(1e-6)));

        // Test projection of zero vector (should be zero)
        let zero = Vec2::zero();
        let proj_zero = zero.project_to(vec);
        assert!(vec2_approx_eq(proj_zero, Vec2::zero(), Some(1e-6)));

        // Test specific case: projection of (1,1) onto (1,0)
        let vec3 = Vec2::new(1.0, 1.0);
        let target = Vec2::new(1.0, 0.0);
        let proj = vec3.project_to(target);
        assert!(vec2_approx_eq(proj, Vec2::new(1.0, 0.0), Some(1e-6)));

        // Test projection with floating point precision
        let vec4 = Vec2::new(1.0, 2.0);
        let target2 = Vec2::new(2.0, 1.0);
        let proj_float = vec4.project_to(target2);
        // Manual calculation: dot = 1*2 + 2*1 = 4, mag² = 2² + 1² = 5, c = 4/5
        // projection = (2, 1) * (4/5) = (8/5, 4/5)
        let expected = Vec2::new(8.0 / 5.0, 4.0 / 5.0);
        assert!(vec2_approx_eq(proj_float, expected, Some(1e-6)));

        // Test with negative components
        let vec5 = Vec2::new(-3.0, 4.0);
        let target3 = Vec2::new(1.0, 1.0);
        let proj_neg = vec5.project_to(target3);
        // dot = -3*1 + 4*1 = 1, mag² = 1² + 1² = 2, c = 1/2
        // projection = (1, 1) * (1/2) = (0.5, 0.5)
        let expected_neg = Vec2::new(0.5, 0.5);
        assert!(vec2_approx_eq(proj_neg, expected_neg, Some(1e-6)));
    }

    #[test]
    fn test_from_array() {
        // Test conversion from array
        let arr = [3.0, 4.0];
        let vec: Vec2 = Vec2::from(arr);
        assert_eq!(vec.x, 3.0);
        assert_eq!(vec.y, 4.0);

        // Test with negative values
        let arr_neg = [-2.5, -1.7];
        let vec_neg: Vec2 = Vec2::from(arr_neg);
        assert_eq!(vec_neg.x, -2.5);
        assert_eq!(vec_neg.y, -1.7);

        // Test with zero values
        let arr_zero = [0.0, 0.0];
        let vec_zero: Vec2 = Vec2::from(arr_zero);
        assert_eq!(vec_zero.x, 0.0);
        assert_eq!(vec_zero.y, 0.0);

        // Test with floating point values
        let arr_float = [0.1, 0.2];
        let vec_float: Vec2 = Vec2::from(arr_float);
        assert!(approx_eq(vec_float.x, 0.1, Some(1e-6)));
        assert!(approx_eq(vec_float.y, 0.2, Some(1e-6)));
    }

    #[test]
    fn test_apply_transformation() {
        let vec = Vec2::new(2.0, 3.0);

        // Test identity transformation
        let identity = Transformation2D::identity();
        let transformed_identity = vec.apply_transformation(identity);
        assert!(vec2_approx_eq(transformed_identity, vec, Some(1e-6)));

        // Test translation
        let translation = Transformation2D::translation(1.0, 2.0);
        let transformed_trans = vec.apply_transformation(translation);
        let expected_trans = Vec2::new(3.0, 5.0); // (2+1, 3+2)
        assert!(vec2_approx_eq(
            transformed_trans,
            expected_trans,
            Some(1e-6)
        ));

        // Test scaling
        let scale = Transformation2D::scale(2.0, 3.0);
        let transformed_scale = vec.apply_transformation(scale);
        let expected_scale = Vec2::new(4.0, 9.0); // (2*2, 3*3)
        assert!(vec2_approx_eq(
            transformed_scale,
            expected_scale,
            Some(1e-6)
        ));

        // Test rotation (90 degrees)
        let rotation = Transformation2D::rotation(PI / 2.0);
        let transformed_rot = vec.apply_transformation(rotation);
        let expected_rot = Vec2::new(-3.0, 2.0); // 90° rotation of (2,3)
        assert!(vec2_approx_eq(transformed_rot, expected_rot, Some(1e-5)));

        // Test reflection across x-axis
        let reflect_x = Transformation2D::reflection_x();
        let transformed_ref_x = vec.apply_transformation(reflect_x);
        let expected_ref_x = Vec2::new(2.0, -3.0);
        assert!(vec2_approx_eq(
            transformed_ref_x,
            expected_ref_x,
            Some(1e-6)
        ));

        // Test reflection across y-axis
        let reflect_y = Transformation2D::reflection_y();
        let transformed_ref_y = vec.apply_transformation(reflect_y);
        let expected_ref_y = Vec2::new(-2.0, 3.0);
        assert!(vec2_approx_eq(
            transformed_ref_y,
            expected_ref_y,
            Some(1e-6)
        ));

        // Test combined transformation (scale then translate)
        let combined =
            Transformation2D::translation(1.0, 1.0).multiply(&Transformation2D::scale(2.0, 2.0));
        let transformed_combined = vec.apply_transformation(combined);
        let expected_combined = Vec2::new(5.0, 7.0); // scale (2,3) to (4,6), then translate to (5,7)
        assert!(vec2_approx_eq(
            transformed_combined,
            expected_combined,
            Some(1e-6)
        ));

        // Test with zero vector
        let zero = Vec2::zero();
        let transformed_zero = zero.apply_transformation(translation);
        let expected_zero_trans = Vec2::new(1.0, 2.0);
        assert!(vec2_approx_eq(
            transformed_zero,
            expected_zero_trans,
            Some(1e-6)
        ));
    }

    #[test]
    fn test_clone_and_copy() {
        let vec = Vec2::new(3.0, 4.0);

        // Test clone
        let cloned = vec.clone();
        assert_eq!(cloned.x, vec.x);
        assert_eq!(cloned.y, vec.y);

        // Test copy (implicit)
        let copied = vec;
        assert_eq!(copied.x, vec.x);
        assert_eq!(copied.y, vec.y);

        // Verify original is still accessible (proves Copy trait works)
        assert_eq!(vec.x, 3.0);
        assert_eq!(vec.y, 4.0);
    }

    #[test]
    fn test_debug_trait() {
        let vec = Vec2::new(3.0, 4.0);
        let debug_string = format!("{:?}", vec);

        // Just verify that debug formatting works without panicking
        // and contains the expected values
        assert!(debug_string.contains("3.0"));
        assert!(debug_string.contains("4.0"));
        assert!(debug_string.contains("Vec2"));
    }

    #[test]
    fn test_edge_cases_and_special_values() {
        // Test with infinity
        let inf_vec = Vec2::new(f32::INFINITY, f32::NEG_INFINITY);
        assert_eq!(inf_vec.x, f32::INFINITY);
        assert_eq!(inf_vec.y, f32::NEG_INFINITY);

        // Test operations with infinity
        let normal_vec = Vec2::new(1.0, 1.0);
        let inf_result = normal_vec.add(inf_vec);
        assert_eq!(inf_result.x, f32::INFINITY);
        assert_eq!(inf_result.y, f32::NEG_INFINITY);

        // Test with NaN
        let nan_vec = Vec2::new(f32::NAN, f32::NAN);
        assert!(nan_vec.x.is_nan());
        assert!(nan_vec.y.is_nan());

        // Test magnitude with very large numbers (but not so large as to overflow)
        let large_vec = Vec2::new(1e10, 1e10);
        let mag = large_vec.magnitude();
        assert!(!mag.is_nan());
        assert!(mag.is_finite());

        // Test dot product edge case
        let unit1 = Vec2::new(1.0, 0.0);
        let unit2 = Vec2::new(0.0, 1.0);
        assert_eq!(unit1.dot_prod(unit2), 0.0);

        // Test projection edge case - projecting onto very small vector
        let tiny_vec = Vec2::new(1e-20, 0.0);
        let normal = Vec2::new(1.0, 1.0);
        let proj = normal.project_to(tiny_vec);
        // Should not panic or produce NaN/Inf (though might be very large due to division)
        // Just check that the calculation completes without panic
        let _ = proj.x;
        let _ = proj.y;
    }

    #[test]
    fn test_mathematical_properties() {
        let a = Vec2::new(2.0, 3.0);
        let b = Vec2::new(1.0, 4.0);
        let c = Vec2::new(-1.0, 2.0);
        let scalar = 2.5;

        // Test associativity of addition: (a + b) + c = a + (b + c)
        let left_assoc = a.add(b).add(c);
        let right_assoc = a.add(b.add(c));
        assert!(vec2_approx_eq(left_assoc, right_assoc, Some(1e-6)));

        // Test commutativity of addition: a + b = b + a
        assert!(vec2_approx_eq(a.add(b), b.add(a), Some(1e-6)));

        // Test distributivity of scaling: s * (a + b) = s * a + s * b
        let left_dist = a.add(b).scale(scalar);
        let right_dist = a.scale(scalar).add(b.scale(scalar));
        assert!(vec2_approx_eq(left_dist, right_dist, Some(1e-6)));

        // Test dot product properties: a · b = b · a
        assert!(approx_eq(a.dot_prod(b), b.dot_prod(a), Some(1e-6)));

        // Test distributivity of dot product: a · (b + c) = a · b + a · c
        let left_dot_dist = a.dot_prod(b.add(c));
        let right_dot_dist = a.dot_prod(b) + a.dot_prod(c);
        assert!(approx_eq(left_dot_dist, right_dot_dist, Some(1e-6)));

        // Test scaling property of dot product: (s * a) · b = s * (a · b)
        let scaled_dot = a.scale(scalar).dot_prod(b);
        let scalar_times_dot = scalar * a.dot_prod(b);
        assert!(approx_eq(scaled_dot, scalar_times_dot, Some(1e-6)));

        // Test magnitude properties: |s * a| = |s| * |a|
        let scaled_magnitude = a.scale(scalar).magnitude();
        let scalar_times_magnitude = scalar.abs() * a.magnitude();
        assert!(approx_eq(
            scaled_magnitude,
            scalar_times_magnitude,
            Some(1e-6)
        ));

        // Test triangle inequality: |a + b| ≤ |a| + |b|
        let sum_magnitude = a.add(b).magnitude();
        let sum_of_magnitudes = a.magnitude() + b.magnitude();
        assert!(sum_magnitude <= sum_of_magnitudes + 1e-6);
    }
}

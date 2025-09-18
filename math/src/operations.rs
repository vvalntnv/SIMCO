pub fn approx_eq(a: f32, b: f32, custom_epsilon: Option<f32>) -> bool {
    let epsilon = if let Some(value) = custom_epsilon {
        value
    } else {
        1e-10
    };

    (a - b).abs() < epsilon
}

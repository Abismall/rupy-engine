use rupy::math::{
    add_vec2, add_vec3, clamp, cross_vec3, dot_vec2, dot_vec3, lerp, mat4_id, mat4_mul,
    normalize_vec2, normalize_vec3, pixel_to_ndc, scale_vec2, scale_vec3,
    spatial::{mat4_inverse, GetValue, Height, Size2D, Size3D, Width},
    subtract_vec2, subtract_vec3,
    vector::{vec3_div, vec3_to_mat4_translation, vec4_multiply},
    Vec2, Vec3, Vec4,
};

#[test]
fn test_mat4_identity() {
    let identity = mat4_id();
    let expected = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    assert_eq!(identity, expected);
}
#[test]
fn test_clamp() {
    // Test clamping when value is below the minimum
    assert_eq!(clamp(0.5, 1.0, 5.0), 1.0);

    // Test clamping when value is above the maximum
    assert_eq!(clamp(6.0, 1.0, 5.0), 5.0);

    // Test clamping when value is within the range
    assert_eq!(clamp(3.0, 1.0, 5.0), 3.0);

    // Test clamping at boundary (min)
    assert_eq!(clamp(1.0, 1.0, 5.0), 1.0);

    // Test clamping at boundary (max)
    assert_eq!(clamp(5.0, 1.0, 5.0), 5.0);
}

#[test]
fn test_lerp() {
    // Test basic linear interpolation
    assert_eq!(lerp(0.0, 10.0, 0.5), 5.0);

    // Test interpolation when t = 0 (should return start)
    assert_eq!(lerp(1.0, 5.0, 0.0), 1.0);

    // Test interpolation when t = 1 (should return end)
    assert_eq!(lerp(1.0, 5.0, 1.0), 5.0);

    // Test interpolation beyond t > 1
    assert_eq!(lerp(1.0, 5.0, 1.5), 7.0);

    // Test interpolation for negative t
    assert_eq!(lerp(1.0, 5.0, -0.5), -1.0);
}

#[test]
fn test_pixel_to_ndc() {
    // Test conversion in the center of the screen
    assert_eq!(pixel_to_ndc(400.0, 300.0, 800.0, 600.0), [0.0, 0.0]);

    // Test conversion for the top-left corner (should be [-1.0, 1.0])
    assert_eq!(pixel_to_ndc(0.0, 0.0, 800.0, 600.0), [-1.0, 1.0]);

    // Test conversion for the bottom-right corner (should be [1.0, -1.0])
    assert_eq!(pixel_to_ndc(800.0, 600.0, 800.0, 600.0), [1.0, -1.0]);

    // Test conversion for a point halfway along the width
    assert_eq!(pixel_to_ndc(400.0, 0.0, 800.0, 600.0), [0.0, 1.0]);

    // Test conversion for a point halfway along the height
    assert_eq!(pixel_to_ndc(0.0, 300.0, 800.0, 600.0), [-1.0, 0.0]);
}
#[test]
fn test_mat4_multiplication() {
    let mat_a = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    let mat_b = [
        [2.0, 3.0, 4.0, 5.0],
        [1.0, 2.0, 3.0, 4.0],
        [0.0, 1.0, 2.0, 3.0],
        [0.0, 0.0, 1.0, 2.0],
    ];
    let result = mat4_mul(mat_a, mat_b);

    let expected = [
        [2.0, 3.0, 4.0, 5.0],
        [1.0, 2.0, 3.0, 4.0],
        [0.0, 1.0, 2.0, 3.0],
        [0.0, 0.0, 1.0, 2.0],
    ];

    assert_eq!(result, expected);
}

#[test]
fn test_mat4_inverse() {
    let matrix = [
        [1.0, 2.0, 3.0, 4.0],
        [0.0, 1.0, 2.0, 3.0],
        [0.0, 0.0, 1.0, 2.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    let inverse_matrix = mat4_inverse(matrix);
    let expected = [
        [1.0, -2.0, 1.0, 0.0],
        [0.0, 1.0, -2.0, 1.0],
        [0.0, 0.0, 1.0, -2.0],
        [0.0, 0.0, 0.0, 1.0],
    ];

    // You might want to verify with floating-point precision, so using an epsilon check
    for i in 0..4 {
        for j in 0..4 {
            assert!((inverse_matrix[i][j] - expected[i][j]).abs() < 1e-6);
        }
    }
}

#[test]
fn test_size2d_operations() {
    let size = Size2D::new(200, 100);

    // Test scaling
    let scaled = size.scale(1.5, 2.0);
    assert_eq!(scaled.width.get(), 300);
    assert_eq!(scaled.height.get(), 200);

    // Test clamping
    let clamped = size.clamp(50, 50, 250, 150);
    assert_eq!(clamped.width.get(), 200);
    assert_eq!(clamped.height.get(), 100);

    // Test interpolation
    let target_size = Size2D::new(300, 200);
    let interpolated = size.interpolate(&target_size, 0.5);
    assert_eq!(interpolated.width.get(), 250);
    assert_eq!(interpolated.height.get(), 150);
}

#[test]
fn test_size3d_operations() {
    let size3d = Size3D::new(100, 200, 300);

    // Test scaling
    let scaled = size3d.scale(2.0, 2.0, 0.5);
    assert_eq!(scaled.size_2d.width.get(), 200);
    assert_eq!(scaled.size_2d.height.get(), 400);
    assert_eq!(scaled.depth.get(), 150);

    // Test clamping
    let clamped = size3d.clamp(50, 100, 150, 250, 300, 400);
    assert_eq!(clamped.size_2d.width.get(), 100);
    assert_eq!(clamped.size_2d.height.get(), 200);
    assert_eq!(clamped.depth.get(), 300);
}

#[test]
fn test_width_arithmetic_operations() {
    let width1 = Width::new(100);
    let width2 = Width::new(50);

    // Test Add
    let result = width1 + width2;
    assert_eq!(result.get(), 150);

    // Test Sub
    let result = width1 - width2;
    assert_eq!(result.get(), 50);

    // Test Mul
    let result = width1 * width2;
    assert_eq!(result.get(), 5000);

    // Test Div
    let result = width1 / width2;
    assert_eq!(result.get(), 2);

    // Test Rem
    let result = width1 % width2;
    assert_eq!(result.get(), 0);
}

#[test]
fn test_height_arithmetic_operations() {
    let height1 = Height::new(100);
    let height2 = Height::new(25);

    // Test Add
    let result = height1 + height2;
    assert_eq!(result.get(), 125);

    // Test Sub
    let result = height1 - height2;
    assert_eq!(result.get(), 75);

    // Test Mul
    let result = height1 * height2;
    assert_eq!(result.get(), 2500);

    // Test Div
    let result = height1 / height2;
    assert_eq!(result.get(), 4);

    // Test Rem
    let result = height1 % height2;
    assert_eq!(result.get(), 0);
}
#[test]
fn test_add_vec2() {
    let a: Vec2 = [1.0, 2.0];
    let b: Vec2 = [3.0, 4.0];
    let result = add_vec2(a, b);
    assert_eq!(result, [4.0, 6.0]);
}

#[test]
fn test_subtract_vec2() {
    let a: Vec2 = [3.0, 4.0];
    let b: Vec2 = [1.0, 2.0];
    let result = subtract_vec2(a, b);
    assert_eq!(result, [2.0, 2.0]);
}

#[test]
fn test_scale_vec2() {
    let v: Vec2 = [1.0, 2.0];
    let result = scale_vec2(v, 2.0);
    assert_eq!(result, [2.0, 4.0]);
}

#[test]
fn test_dot_vec2() {
    let a: Vec2 = [1.0, 2.0];
    let b: Vec2 = [3.0, 4.0];
    let result = dot_vec2(a, b);
    assert_eq!(result, 11.0);
}

#[test]
fn test_normalize_vec2() {
    let v: Vec2 = [3.0, 4.0];
    let result = normalize_vec2(v);
    let expected: Vec2 = [0.6, 0.8];
    assert!((result[0] - expected[0]).abs() < 1e-6);
    assert!((result[1] - expected[1]).abs() < 1e-6);
}

#[test]
fn test_add_vec3() {
    let a: Vec3 = [1.0, 2.0, 3.0];
    let b: Vec3 = [4.0, 5.0, 6.0];
    let result = add_vec3(a, b);
    assert_eq!(result, [5.0, 7.0, 9.0]);
}

#[test]
fn test_subtract_vec3() {
    let a: Vec3 = [4.0, 5.0, 6.0];
    let b: Vec3 = [1.0, 2.0, 3.0];
    let result = subtract_vec3(a, b);
    assert_eq!(result, [3.0, 3.0, 3.0]);
}

#[test]
fn test_scale_vec3() {
    let v: Vec3 = [1.0, 2.0, 3.0];
    let result = scale_vec3(v, 2.0);
    assert_eq!(result, [2.0, 4.0, 6.0]);
}

#[test]
fn test_dot_vec3() {
    let a: Vec3 = [1.0, 2.0, 3.0];
    let b: Vec3 = [4.0, 5.0, 6.0];
    let result = dot_vec3(a, b);
    assert_eq!(result, 32.0);
}

#[test]
fn test_cross_vec3() {
    let a: Vec3 = [1.0, 0.0, 0.0];
    let b: Vec3 = [0.0, 1.0, 0.0];
    let result = cross_vec3(a, b);
    assert_eq!(result, [0.0, 0.0, 1.0]);
}

#[test]
fn test_normalize_vec3() {
    let v: Vec3 = [3.0, 4.0, 0.0];
    let result = normalize_vec3(v);
    let expected: Vec3 = [0.6, 0.8, 0.0];
    assert!((result[0] - expected[0]).abs() < 1e-6);
    assert!((result[1] - expected[1]).abs() < 1e-6);
}

#[test]
fn test_vec4_multiply() {
    let a: Vec4 = [1.0, 2.0, 3.0, 4.0];
    let b: Vec4 = [4.0, 3.0, 2.0, 1.0];
    let result = vec4_multiply(a, b);
    assert_eq!(result, [4.0, 6.0, 6.0, 4.0]);
}

#[test]
fn test_vec3_to_mat4_translation() {
    let v: Vec3 = [1.0, 2.0, 3.0];
    let result = vec3_to_mat4_translation(v);
    let expected = [
        [1.0, 0.0, 0.0, 1.0],
        [0.0, 1.0, 0.0, 2.0],
        [0.0, 0.0, 1.0, 3.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    assert_eq!(result, expected);
}

#[test]
fn test_vec3_div() {
    let v: Vec3 = [6.0, 12.0, 18.0];
    let result = vec3_div(v, 3.0);
    assert_eq!(result, [2.0, 4.0, 6.0]);
}

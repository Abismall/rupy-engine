struct Uniforms {
    model: mat4x4<f32>,            // Model matrix for object transformations
    view_proj: mat4x4<f32>,        // Combined View-Projection matrix
    color: vec4<f32>,              // Global color multiplier
    light_color: vec4<f32>,        // Unused here but kept for consistency
    light_position: vec4<f32>,     // Unused here but kept for consistency
    view_position: vec3<f32>,      // Unused here but kept for consistency
    ambient_strength: f32,         // Unused here but kept for consistency
    diffuse_strength: f32,         // Unused here but kept for consistency
    specular_strength: f32,        // Unused here but kept for consistency
    shininess: f32,                // Unused here but kept for consistency
};

@binding(0) @group(0) var<uniform> uniforms: Uniforms;

@fragment
fn fs_main(
    @location(0) vertex_color: vec4<f32> // Interpolated color from vertex shader
) -> @location(0) vec4<f32> {
    // Combine vertex color with uniform color
    return vertex_color * uniforms.color;
}

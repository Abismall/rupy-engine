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

struct VertexInput {
    @location(0) position: vec2<f32>,    // 2D vertex position
    @location(1) color: vec4<f32>,       // Vertex color
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,  // Clip-space position
    @location(0) color: vec4<f32>,           // Interpolated color
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    // Apply model matrix for transformations, and then view-projection
    let world_position = uniforms.model * vec4<f32>(input.position, 0.0, 1.0);
    output.position = uniforms.view_proj * world_position;

    // Pass the vertex color to the fragment shader
    output.color = input.color;

    return output;
}

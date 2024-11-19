struct Uniforms {
    model: mat4x4<f32>,          // Model matrix for object-specific transformations
    view_proj: mat4x4<f32>,      // View-Projection matrix for camera transformations
    color: vec4<f32>,            // Global color
    light_color: vec4<f32>,      // Light color (unused here, but kept for structure matching)
    light_position: vec4<f32>,   // Light position (unused here, but kept for structure matching)
    view_position: vec3<f32>,    // View position (unused here, but kept for structure matching)
    ambient_strength: f32,       // Ambient strength (unused here, but kept for structure matching)
    diffuse_strength: f32,       // Diffuse strength (unused here, but kept for structure matching)
    specular_strength: f32,      // Specular strength (unused here, but kept for structure matching)
    shininess: f32,              // Shininess factor (unused here, but kept for structure matching)
};

@binding(0) @group(0) var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec2<f32>,     // 2D vertex position
    @location(1) color: vec4<f32>,        // Vertex color
    @location(2) tex_coords: vec2<f32>,   // Texture coordinates
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>, // Clip-space position
    @location(0) color: vec4<f32>,          // Interpolated color
    @location(1) tex_coords: vec2<f32>,     // Interpolated texture coordinates
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    // Apply model transformation followed by view-projection transformation
    let model_view_proj = uniforms.view_proj * uniforms.model;
    output.position = model_view_proj * vec4<f32>(input.position, 0.0, 1.0);

    // Pass through the vertex color and texture coordinates
    output.color = input.color;
    output.tex_coords = input.tex_coords;

    return output;
}

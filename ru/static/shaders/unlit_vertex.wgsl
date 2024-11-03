struct Uniforms {
    model: mat4x4<f32>,            // 4x4 model matrix
    view_projection: mat4x4<f32>,  // 4x4 view-projection matrix
    color: vec4<f32>,              // RGBA color
};

@binding(0) @group(0) var<uniform> uniforms: Uniforms;

// Vertex input and output structures
struct VertexInput {
    @location(0) position: vec3<f32>,    // Vertex position
    @location(1) color: vec3<f32>,       // Vertex color
    @location(2) normal: vec3<f32>,      // Normal vector
    @location(3) tex_coords: vec2<f32>,  // Texture coordinates
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,  // Position for the next stage
    @location(0) color: vec3<f32>,           // Pass-through color
};

// Vertex shader
@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    // Apply model and view-projection transformations
    output.position = uniforms.view_projection * uniforms.model * vec4<f32>(input.position, 1.0);
    output.color = input.color;
    return output;
}

// Uniform structure for model, view-projection matrix, and color
struct Uniforms {
    model: mat4x4<f32>,      // Model matrix
    view_proj: mat4x4<f32>,  // View-projection matrix
    color: vec4<f32>,        // RGBA color for the object
};

@binding(0) @group(0) var<uniform> uniforms: Uniforms;

// Input structure for vertex processing
struct VertexInput {
    @location(0) position: vec3<f32>,  // Vertex position in model space
    @location(1) normal: vec3<f32>,    // Vertex normal for calculating outline
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,  // Output position in clip space
};
@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return uniforms.color; // Use the color provided by the uniforms, including alpha for transparency
}

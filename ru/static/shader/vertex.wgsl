// Updated Uniforms for the model matrix, view-projection matrix, and color
struct Uniforms {
    model: mat4x4<f32>,      // 4x4 model matrix
    view_proj: mat4x4<f32>,  // 4x4 view-projection matrix
    color: vec4<f32>,        // RGBA color
};

@binding(0) @group(0) var<uniform> uniforms: Uniforms;

// Vertex input structure
struct VertexInput {
    @location(0) position: vec3<f32>,  // Vertex position in model space
    @location(1) color: vec3<f32>,     // Vertex color
};

// Vertex output structure
struct VertexOutput {
    @builtin(position) position: vec4<f32>,  // Output position in clip space
    @location(0) color: vec3<f32>,           // Pass through the color to fragment shader
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    
    // Apply model matrix first, then view-projection matrix
    output.position = uniforms.view_proj * uniforms.model * vec4<f32>(input.position, 1.0);
    
    // Pass the vertex color through to the fragment shader
    output.color = input.color;
    return output;
}


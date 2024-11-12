struct Uniforms {
    view_proj: mat4x4<f32>, // Projection matrix for 2D rendering
};

@binding(0) @group(0) var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec2<f32>, // Position in 2D space (x, y)
    @location(1) color: vec4<f32>,    // Color of the vertex (RGBA)
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>, // Position in clip space
    @location(0) color: vec4<f32>,          // Color passed to the fragment shader
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    // Convert 2D position into 4D homogeneous coordinates for rendering
    output.position = uniforms.view_proj * vec4<f32>(input.position, 0.0, 1.0);
    output.color = input.color; // Pass through the color
    return output;
}

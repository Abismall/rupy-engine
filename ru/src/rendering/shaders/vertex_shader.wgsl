// vertex_shader.wgsl

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@group(0) @binding(0)
var<uniform> transform: mat4x4<f32>; // Uniform buffer containing the perspective matrix

@vertex
fn main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    // Correct multiplication to transform the vertex into the 3D space
    output.position = transform * vec4(input.position, 1.0); 
    output.color = input.color; 
    return output;
}

struct VertexInput {
    @location(0) position: vec3<f32>,  // Vertex position
    @location(1) tex_coords: vec2<f32>, // Texture coordinates
};

struct Uniforms {
    view_projection: mat4x4<f32>,
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>, // Output clip space position
    @location(0) frag_tex_coords: vec2<f32>,     // Pass-through texture coordinates
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.clip_position = uniforms.view_projection * vec4<f32>(input.position, 1.0);
    output.frag_tex_coords = input.tex_coords; // Pass texture coords to fragment shader
    return output;
}

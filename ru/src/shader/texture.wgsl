struct VertexInput {
    @location(0) position: vec3<f32>,   // Vertex position
    @location(1) tex_coords: vec2<f32>, // Texture coordinates
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,  // Clip-space position for the vertex
    @location(0) tex_coords: vec2<f32>,      // Passing texture coordinates to fragment shader
};

@vertex
fn v_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.position = vec4<f32>(input.position, 1.0); // Set vertex position, convert vec3 to vec4
    output.tex_coords = input.tex_coords;            // Pass texture coordinates
    return output;
}

@group(0) @binding(0)
var texture_sampler: sampler;          // Sampler for the texture
@group(0) @binding(1)
var texture: texture_2d<f32>;          // 2D texture bound at group 0, binding 1

@fragment
fn f_main(@location(0) tex_coords: vec2<f32>) -> @location(0) vec4<f32> {
    // Sample the texture using tex_coords and the sampler, return the color
    return textureSample(texture, texture_sampler, tex_coords);
}

struct Uniforms {
    model: mat4x4<f32>,           // 4x4 model matrix
    view_proj: mat4x4<f32>,       // 4x4 view-projection matrix
    color: vec4<f32>,             // RGBA color
};

@binding(0) @group(0) var<uniform> uniforms: Uniforms;

@binding(0) @group(1) var texture_sampler: sampler;

@binding(1) @group(1) var texture_image: texture_2d<f32>;

struct VertexInput {
    @location(0) position: vec3<f32>,    // Vertex position
    @location(1) color: vec3<f32>,       // Vertex color
    @location(2) normal: vec3<f32>,      // Normal vector
    @location(3) tex_coords: vec2<f32>,  // Texture coordinates
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,  // Position for the next stage
    @location(0) color: vec3<f32>,           // Pass-through color
    @location(1) tex_coords: vec2<f32>,      // Pass-through texture coordinates
};

@fragment
fn fs_main(
    @location(0) vertex_color: vec3<f32>,
    @location(1) tex_coords: vec2<f32>
) -> @location(0) vec4<f32> {
    let texture_color = textureSample(texture_image, texture_sampler, tex_coords).rgb;

    let final_color = mix(texture_color, uniforms.color.rgb * texture_color, uniforms.color.a);

    return vec4<f32>(final_color, uniforms.color.a);
}

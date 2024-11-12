struct Uniforms {
    model: mat4x4<f32>,           // 4x4 model matrix
    view_proj: mat4x4<f32>,       // 4x4 view-projection matrix
    color: vec4<f32>,             // RGBA color
};

@binding(0) @group(1) var texture_sampler: sampler;
@binding(1) @group(1) var texture_image: texture_2d<f32>;

@binding(0) @group(0) var<uniform> uniforms: Uniforms;

@fragment
fn fs_main(
    @location(0) vertex_color: vec3<f32>,
    @location(1) tex_coords: vec2<f32>
) -> @location(0) vec4<f32> {
    // Sample the texture color at the given coordinates
    let texture_color = textureSample(texture_image, texture_sampler, tex_coords).rgb;

    // Combine the texture color with the vertex color and the uniform color
    // Mix the uniform color with the texture color using the uniform alpha value
    let final_color = mix(texture_color, uniforms.color.rgb * vertex_color, uniforms.color.a);

    return vec4<f32>(final_color, uniforms.color.a);
}

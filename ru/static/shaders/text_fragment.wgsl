struct FragmentInput {
    @location(0) frag_tex_coords: vec2<f32>, // Interpolated texture coordinates
};

@group(0) @binding(1) var text_texture: texture_2d<f32>;
@group(0) @binding(2) var text_sampler: sampler;

struct Uniforms {
    color: vec4<f32>, // Color for the text
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

@fragment
fn fs_main(input: FragmentInput) -> @location(0) vec4<f32> {
    // Sample the text texture using the interpolated texture coordinates
    let sampled_color = textureSample(text_texture, text_sampler, input.frag_tex_coords);

    // Multiply sampled color's alpha by the uniform color
    return vec4<f32>(uniforms.color.rgb, uniforms.color.a * sampled_color.a);
}

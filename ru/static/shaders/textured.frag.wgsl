
struct FragmentInput {
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,
};

@group(0) @binding(2)
var my_texture: texture_2d<f32>;

@group(0) @binding(3)
var my_sampler: sampler;

@fragment
fn fs_main(input: FragmentInput) -> @location(0) vec4<f32> {
    let tex_color = textureSample(my_texture, my_sampler, input.uv);
    return tex_color * input.color;
}

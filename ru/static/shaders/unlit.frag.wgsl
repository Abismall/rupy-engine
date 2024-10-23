struct FragmentInput {
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>, 
};

@fragment
fn fs_main(input: FragmentInput) -> @location(0) vec4<f32> {
    return input.color;
}

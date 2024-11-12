struct FragmentInput {
    @location(0) color: vec4<f32>, // Color passed from the vertex shader
};

@fragment
fn fs_main(input: FragmentInput) -> @location(0) vec4<f32> {
    return input.color; // Use the color from the vertex shader
}

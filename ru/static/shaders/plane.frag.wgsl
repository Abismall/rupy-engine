struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>, // Changed to vec4<f32>
    @location(1) uv: vec2<f32>,    // Pass uv to fragment shader if needed
};

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return input.color; 
}

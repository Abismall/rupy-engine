struct VertexOutput {
    @builtin(position) position: vec4<f32>, // Output the position of the vertex
    @location(0) color: vec3<f32>,          // Output the color at location 0
};

@vertex
fn vs_main(@location(0) position: vec3<f32>, @location(1) color: vec3<f32>) -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4<f32>(position, 1.0); // Convert vec3 to vec4 with w = 1.0
    out.color = color;                       // Pass the input color directly to output
    return out;
}

@fragment
fn fs_main(@location(0) color: vec3<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(color, 1.0); // Output the color with full opacity
}
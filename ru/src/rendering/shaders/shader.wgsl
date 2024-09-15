// Vertex shader
@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4<f32> {
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(0.0, 0.5),   // Top vertex
        vec2<f32>(-0.5, -0.5), // Bottom left vertex
        vec2<f32>(0.5, -0.5),  // Bottom right vertex
    );

    // Return the position of the vertex
    let position = positions[vertex_index];
    return vec4<f32>(position, 0.0, 1.0);
}

// Fragment shader
@fragment
fn fs_main() -> @location(0) vec4<f32> {
    // Output a solid green color
    return vec4<f32>(0.0, 1.0, 0.0, 1.0);
}
// Updated Uniforms for the model matrix, view-projection matrix, and color
struct Uniforms {
    model: mat4x4<f32>,      // 4x4 model matrix
    view_proj: mat4x4<f32>,  // 4x4 view-projection matrix
    color: vec4<f32>,        // RGBA color
};

@binding(0) @group(0) var<uniform> uniforms: Uniforms;



@fragment
fn fs_main(@location(0) vertex_color: vec3<f32>) -> @location(0) vec4<f32> {
    // Combine the vertex color with the uniform color
    let final_color = uniforms.color.rgb * vertex_color;
    
    // Return the combined color as the fragment output
    return vec4<f32>(final_color, uniforms.color.a);  // Use uniform alpha for output
}
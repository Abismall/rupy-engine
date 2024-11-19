struct Uniforms {
    model: mat4x4<f32>,          // Model matrix (not used in the fragment shader)
    view_proj: mat4x4<f32>,      // View-Projection matrix (not used in the fragment shader)
    color: vec4<f32>,            // Global color multiplier
    light_color: vec4<f32>,      // Light color (unused here, but kept for structure matching)
    light_position: vec4<f32>,   // Light position (unused here, but kept for structure matching)
    view_position: vec3<f32>,    // View position (unused here, but kept for structure matching)
    ambient_strength: f32,       // Ambient strength (unused here, but kept for structure matching)
    diffuse_strength: f32,       // Diffuse strength (unused here, but kept for structure matching)
    specular_strength: f32,      // Specular strength (unused here, but kept for structure matching)
    shininess: f32,              // Shininess factor (unused here, but kept for structure matching)
};

@binding(0) @group(0) var<uniform> uniforms: Uniforms;
@binding(0) @group(1) var texture_sampler: sampler;          // Texture sampler
@binding(1) @group(1) var texture_image: texture_2d<f32>;    // Texture resource

@fragment
fn fs_main(
    @location(0) color: vec4<f32>,        // Interpolated vertex color
    @location(1) tex_coords: vec2<f32>   // Interpolated texture coordinates
) -> @location(0) vec4<f32> {
    // Sample the texture
    let texture_color: vec4<f32> = textureSample(texture_image, texture_sampler, tex_coords);

    // Combine the vertex color, texture color, and global uniform color
    return color * texture_color * uniforms.color;
}

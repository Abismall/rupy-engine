struct Uniforms {
    model: mat4x4<f32>,           // 4x4 model matrix
    view_proj: mat4x4<f32>,       // 4x4 view-projection matrix
    color: vec4<f32>,             // RGBA color
    light_position: vec4<f32>,    // Light position in world space
    light_color: vec4<f32>,       // Light color (RGBA)
    view_position: vec3<f32>,     // Position of the camera (view space)
    ambient_strength: f32,        // Strength of ambient light
    diffuse_strength: f32,        // Strength of diffuse light
    specular_strength: f32,       // Strength of specular light
    shininess: f32,               // Shininess factor for specular highlight
};

@binding(0) @group(0) var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,    // Vertex position
    @location(1) color: vec3<f32>,       // Vertex color
    @location(2) normal: vec3<f32>,      // Normal vector
    @location(3) tex_coords: vec2<f32>,  // Texture coordinates
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,  // Position for the next stage
    @location(0) color: vec3<f32>,           // Pass-through color
    @location(1) tex_coords: vec2<f32>,      // Pass-through texture coordinates
    @location(2) world_position: vec3<f32>,  // World position for lighting calculation
    @location(3) normal: vec3<f32>,          // Normal vector for lighting calculation
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    // Calculate world position of the vertex
    let world_position = (uniforms.model * vec4<f32>(input.position, 1.0)).xyz;

    // Calculate the final clip-space position by applying model and view-projection matrices
    output.position = uniforms.view_proj * vec4<f32>(world_position, 1.0);

    // Pass color, texture coordinates, world position, and normal to the fragment shader
    output.color = input.color;
    output.tex_coords = input.tex_coords;
    output.world_position = world_position;
    output.normal = normalize((uniforms.model * vec4<f32>(input.normal, 0.0)).xyz);

    return output;
}
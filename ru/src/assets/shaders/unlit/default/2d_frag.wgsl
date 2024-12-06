struct Uniforms {
    model: mat4x4<f32>,           // 4x4 model matrix
    view_proj: mat4x4<f32>,       // 4x4 view-projection matrix
    color: vec4<f32>,             // RGBA color
    light_position: vec3<f32>,    // Light position in world space
    light_color: vec4<f32>,       // Light color (RGBA)
    view_position: vec3<f32>,     // Position of the camera (view space)
    ambient_strength: f32,        // Strength of ambient light
    diffuse_strength: f32,        // Strength of diffuse light
    specular_strength: f32,       // Strength of specular light
    shininess: f32,               // Shininess factor for specular highlight
};

@binding(0) @group(0) var<uniform> uniforms: Uniforms;

@binding(0) @group(1) var texture_sampler: sampler;
@binding(1) @group(1) var texture_image: texture_2d<f32>;



@fragment
fn fs_main(
    @location(0) vertex_color: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) world_position: vec3<f32>,
    @location(3) normal: vec3<f32>
) -> @location(0) vec4<f32> {
    // Sample the texture color at the given coordinates
    let texture_color: vec3<f32> = textureSample(texture_image, texture_sampler, tex_coords).rgb;

    // Normalize the normal vector
    let norm: vec3<f32> = normalize(normal);

    // Calculate the light direction and view direction
    let light_dir: vec3<f32> = normalize(uniforms.light_position.xyz - world_position);
    let view_dir: vec3<f32> = normalize(uniforms.view_position - world_position);

    // Ambient component
    let ambient: vec3<f32> = uniforms.ambient_strength * uniforms.light_color.xyz;

    // Diffuse component (Lambertian reflectance)
    let diffuse_strength: f32 = max(dot(norm, light_dir), 0.0);
    let diffuse: vec3<f32> = uniforms.diffuse_strength * diffuse_strength * uniforms.light_color.xyz;

    // Specular component (Phong model)
    let reflect_dir: vec3<f32> = reflect(-light_dir, norm);
    let specular_strength: f32 = pow(max(dot(view_dir, reflect_dir), 0.0), uniforms.shininess);
    let specular: vec3<f32> = uniforms.specular_strength * specular_strength * uniforms.light_color.xyz;

    // Combine lighting components with the texture and vertex color
    let lighting: vec3<f32> = ambient + diffuse + specular;
    let final_color: vec3<f32> = texture_color * vertex_color * lighting;

    return vec4<f32>(final_color, uniforms.color.a);
}
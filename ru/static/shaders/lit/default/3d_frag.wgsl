struct Uniforms {
    model: mat4x4<f32>,           // 64 bytes
    view_proj: mat4x4<f32>,       // 64 bytes
    color: vec4<f32>,             // 16 bytes
    light_color: vec4<f32>,       // 16 bytes
    light_position: vec4<f32>,    // 16 bytes
    view_position: vec3<f32>,     // 12 bytes
    ambient_strength: f32,        // 4 bytes
    diffuse_strength: f32,        // 4 bytes
    specular_strength: f32,       // 4 bytes
    shininess: f32,               // 4 bytes
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
    let texture_color: vec3<f32> = textureSample(texture_image, texture_sampler, tex_coords).rgb;
    let norm: vec3<f32> = normalize(normal);

    var light_dir: vec3<f32>;
    if uniforms.light_position.w == 1.0 {
        light_dir = normalize(uniforms.light_position.xyz - world_position);
    } else {
        light_dir = normalize(uniforms.light_position.xyz);
    }

    let view_dir: vec3<f32> = normalize(uniforms.view_position - world_position);

    let ambient: vec3<f32> = uniforms.ambient_strength * uniforms.light_color.xyz;

    let diffuse_strength: f32 = max(dot(norm, light_dir), 0.0);
    let distance = length(uniforms.light_position.xyz - world_position);
    var attenuation: f32 = 1.0; // Initialize attenuation with a default value
    if uniforms.light_position.w == 1.0 {
        // Attenuation for positional lights
        attenuation = 1.0 / (1.0 + 0.09 * distance + 0.032 * distance * distance);
    }
    
    let diffuse: vec3<f32> = uniforms.diffuse_strength * diffuse_strength * uniforms.light_color.xyz * attenuation;

    let reflect_dir: vec3<f32> = reflect(-light_dir, norm);
    let specular_strength: f32 = pow(max(dot(view_dir, reflect_dir), 0.0), uniforms.shininess);
    let specular: vec3<f32> = uniforms.specular_strength * specular_strength * uniforms.light_color.xyz * attenuation;

    let lighting: vec3<f32> = ambient + diffuse + specular;

    let final_color: vec3<f32> = texture_color * vertex_color * lighting * uniforms.color.rgb;

    return vec4<f32>(final_color, uniforms.color.a);
}

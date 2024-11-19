struct Uniforms {
    model: mat4x4<f32>,
    view_proj: mat4x4<f32>,
    color: vec4<f32>,
    light_color: vec4<f32>,
    light_position: vec4<f32>,
    view_position: vec3<f32>,
    ambient_strength: f32,
    diffuse_strength: f32,
    specular_strength: f32,
    shininess: f32,
};

@binding(0) @group(0) var<uniform> uniforms: Uniforms;

@fragment
fn fs_main(
    @location(0) vertex_color: vec3<f32>,     // Vertex color
    @location(1) world_position: vec3<f32>,   // World position for lighting
    @location(2) normal: vec3<f32>            // Normal vector for lighting
) -> @location(0) vec4<f32> {
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
    var attenuation: f32 = 1.0;
    if uniforms.light_position.w == 1.0 {
        attenuation = 1.0 / (1.0 + 0.09 * distance + 0.032 * distance * distance);
    }
    let diffuse: vec3<f32> = uniforms.diffuse_strength * diffuse_strength * uniforms.light_color.xyz * attenuation;

    let reflect_dir: vec3<f32> = reflect(-light_dir, norm);
    let specular_strength: f32 = pow(max(dot(view_dir, reflect_dir), 0.0), uniforms.shininess);
    let specular: vec3<f32> = uniforms.specular_strength * specular_strength * uniforms.light_color.xyz * attenuation;

    let lighting: vec3<f32> = ambient + diffuse + specular;

    let final_color: vec3<f32> = vertex_color * lighting * uniforms.color.rgb;

    return vec4<f32>(final_color, uniforms.color.a);
}

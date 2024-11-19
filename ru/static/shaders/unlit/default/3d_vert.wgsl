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

struct VertexInput {
    @location(0) position: vec3<f32>,   
    @location(1) color: vec4<f32>,      
    @location(2) normal: vec3<f32>,     
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,  
    @location(0) color: vec3<f32>,         
    @location(1) world_position: vec3<f32>, 
    @location(2) normal: vec3<f32>,         
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    let world_position = (uniforms.model * vec4<f32>(input.position, 1.0)).xyz;

    output.position = uniforms.view_proj * vec4<f32>(world_position, 1.0);

    output.color = input.color.rgb;
    output.world_position = world_position;
    output.normal = normalize((uniforms.model * vec4<f32>(input.normal, 0.0)).xyz);

    return output;
}

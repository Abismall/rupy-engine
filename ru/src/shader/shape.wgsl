struct Uniforms {
    view_proj: mat4x4<f32>,
}

@binding(0) @group(0) var<uniform> uniforms: Uniforms;

struct ModelUniforms {
    model: mat4x4<f32>,
}

@binding(1) @group(0) var<uniform> model_uniforms: ModelUniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) normal: vec3<f32>
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) normal: vec3<f32>
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    let model_pos = model_uniforms.model * vec4<f32>(input.position, 1.0);
    output.position = uniforms.view_proj * model_pos;
    output.color = input.color;
    output.normal = (model_uniforms.model * vec4<f32>(input.normal, 0.0)).xyz;
    return output;
};

@fragment
fn fs_main(
    @location(0) color: vec3<f32>,
    @location(1) normal: vec3<f32>
) -> @location(0) vec4<f32> {
    let light_dir = normalize(vec3<f32>(0.0, 0.0, -1.0));
    let lighting = max(dot(normalize(normal), light_dir), 0.0);
    let final_color = color * lighting;
    return vec4<f32>(final_color, 1.0);
};

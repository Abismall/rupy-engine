struct ColorUniform {
    rgba: vec4<f32>,
};
struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct Uniforms {
    view_projection: mat4x4<f32>,
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};
@group(1) @binding(0) var<uniform> color: ColorUniform;

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return color.rgba;
}


@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct Uniforms {
    model: mat4x4<f32>,
    view: mat4x4<f32>,
    projection: mat4x4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>, // Output to the rasterizer
    @location(0) color: vec4<f32>,               // Color passed to fragment shader
};

@vertex
fn vs_main(@location(0) position: vec3<f32>, @location(1) color: vec3<f32>) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = uniforms.projection * uniforms.view * uniforms.model * vec4(position, 1.0);
    out.color = vec4(color, 1.0); // Pass color to fragment shader
    return out;
}

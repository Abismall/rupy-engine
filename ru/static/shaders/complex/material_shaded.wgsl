// Vertex shader example (WGSL)
struct CameraUniforms {
    view_projection_matrix: mat4x4<f32>,
};

@group(0) @binding(0) var<uniform> camera: CameraUniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) vColor: vec4<f32>,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.position = camera.view_projection_matrix * vec4<f32>(input.position, 1.0);
    output.vColor = input.color;
    return output;
}

// Fragment shader example (WGSL)
struct FragmentInput {
    @location(0) vColor: vec4<f32>,
};

@fragment
fn fs_main(input: FragmentInput) -> @location(0) vec4<f32> {
    return input.vColor;
}

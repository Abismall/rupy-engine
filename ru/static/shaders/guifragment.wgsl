

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>, // Position in NDC space
    @location(0) color: vec3<f32>,
    @location(1) rect: vec4<f32>, // Rect passed through in NDC space
    @location(2) border_color: vec3<f32>,
}

@vertex
fn vertex(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(model.position, 1.0);
    out.color = model.color;
    out.rect = model.rect;
    out.border_color = model.border_color;

    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // Border thickness (NDC space)
    let border_thickness: f32 = 0.01; 

    // Get the fragment's NDC position
    let ndc_pos = in.clip_position.xy / in.clip_position.w;

    // Define the border logic using the NDC positions
    if (ndc_pos.x > in.rect.x - border_thickness && ndc_pos.x < in.rect.x + border_thickness || 
        ndc_pos.x > in.rect.z - border_thickness && ndc_pos.x < in.rect.z + border_thickness || 
        ndc_pos.y > in.rect.y - border_thickness && ndc_pos.y < in.rect.y + border_thickness || 
        ndc_pos.y > in.rect.w - border_thickness && ndc_pos.y < in.rect.w + border_thickness) 
    {
        return vec4<f32>(in.border_color, 1.0);
    }

    return vec4<f32>(in.color, 1.0);
}

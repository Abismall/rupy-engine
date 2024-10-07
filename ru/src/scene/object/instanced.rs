use wgpu::{Buffer, RenderPass};

pub struct InstancedMesh {
    pub instance_buffer: Buffer, // Buffer to hold instance data (transforms, etc.)
    pub num_instances: u32,      // Number of instances to render
}

impl InstancedMesh {
    pub fn new(instance_buffer: Buffer, num_instances: u32) -> Self {
        Self {
            instance_buffer,
            num_instances,
        }
    }

    // Future instancing logic to manage instance data
}

pub trait InstancedRenderable {
    fn render_instanced(
        &self,
        render_pass: &mut RenderPass,
        instance_buffer: &Buffer,
        num_instances: u32,
    );
}

pub mod controller;
pub mod frustum;
pub mod handler;
pub mod projection;
use cgmath::{Angle, InnerSpace, Matrix4, Point3, Rad, Vector3};

#[derive(Debug)]
pub struct Camera {
    pub position: Point3<f32>,
    yaw: Rad<f32>,
    pitch: Rad<f32>,
}

impl Camera {
    pub fn new<V: Into<Point3<f32>>, Y: Into<Rad<f32>>, P: Into<Rad<f32>>>(
        position: V,
        yaw: Y,
        pitch: P,
    ) -> Self {
        Self {
            position: position.into(),
            yaw: yaw.into(),
            pitch: pitch.into(),
        }
    }
    pub fn calculate_vectors(&self) -> (Vector3<f32>, Vector3<f32>, Vector3<f32>) {
        let forward = Vector3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        )
        .normalize();
        let right = forward.cross(Vector3::unit_y()).normalize();
        let up = right.cross(forward).normalize();

        (forward, right, up)
    }
    pub fn calc_view_matrix(&self) -> Matrix4<f32> {
        let forward = Vector3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        )
        .normalize();

        let target = self.position + forward;
        let up = Vector3::unit_y();

        Matrix4::look_at_rh(self.position, target, up)
    }
}

impl Camera {
    pub fn create_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("bg:layout:camera"),
        })
    }

    pub fn create_bind_group(
        device: &wgpu::Device,
        camera_buffer: &wgpu::Buffer,
    ) -> wgpu::BindGroup {
        let layout = Self::create_bind_group_layout(device);
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("bg:camera"),
        })
    }
}

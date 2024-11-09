#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub position: Point3<f32>,
    pub target: Point3<f32>,
    pub up: nalgebra::Vector3<f32>,
    pub proj_matrix: Matrix4<f32>,
    pub aspect_ratio: f32,
    pub fov_y: f32,
    pub near: f32,
    pub far: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
    pub is_panning: bool,
    pub move_speed: f32,
    pub accesseleration: f32,
}
impl Camera {
    pub fn new(
        position: Point3<f32>,
        target: Point3<f32>,
        up: nalgebra::Vector3<f32>,
        aspect_ratio: f32,
        accesseleration: f32,
        fov_y: f32,
        near: f32,
        far: f32,
    ) -> Self {
        let direction = (position - target).normalize();
        let distance = (position - target).magnitude();
        let yaw = direction.z.atan2(direction.x);
        let pitch = direction.y.asin();
        Camera {
            position,
            target,
            up,
            aspect_ratio,
            fov_y,
            near,
            far,
            yaw,
            pitch,
            accesseleration,
            distance,
            is_panning: false,
            move_speed: 25.0,
            proj_matrix: Default::default(),
        }
    }
    pub fn set_orthographic_projection(
        &mut self,
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) {
        self.proj_matrix = Matrix4::new_orthographic(left, right, bottom, top, near, far);
    }

    pub fn set_perspective_projection(
        &mut self,
        fov_y: f32,
        aspect_ratio: f32,
        near: f32,
        far: f32,
    ) {
        self.proj_matrix = Matrix4::new_perspective(aspect_ratio, fov_y, near, far);
    }
    pub fn view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(&self.position, &self.target, &self.up)
    }
    pub fn projection_matrix(&self) -> Matrix4<f32> {
        Matrix4::new_perspective(self.aspect_ratio, self.fov_y, self.near, self.far)
    }
    pub fn view_projection_matrix(&self) -> Matrix4<f32> {
        self.projection_matrix() * self.view_matrix()
    }
    pub fn update_camera_position(&mut self) {
        let x = self.distance * self.pitch.cos() * self.yaw.cos();
        let y = self.distance * self.pitch.sin();
        let z = self.distance * self.pitch.cos() * self.yaw.sin();
        let offset = Vector3::new(x, y, z);
        self.position = self.target + offset;
    }
    pub fn rotate(&mut self, delta_yaw: f32, delta_pitch: f32) {
        self.yaw += delta_yaw;
        self.pitch = (self.pitch + delta_pitch).clamp(-1.5, 1.5);
        self.update_camera_position();
    }
    pub fn zoom(&mut self, amount: f32) {
        self.distance = (self.distance - amount).clamp(0.5, 100.0);
        self.update_camera_position();
    }
    pub fn pan(&mut self, delta_x: f32, delta_y: f32) {
        let right = Vector3::new(self.yaw.sin(), 0.0, -self.yaw.cos()).normalize();
        let up = self.up;

        self.target += right * delta_x - up * delta_y;

        self.update_camera_position();
    }
    pub fn move_forward(&mut self, delta_time: f32) {
        let forward = Vector3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        )
        .normalize();
        self.target += forward * self.move_speed * delta_time * self.accesseleration;
        self.update_camera_position();
    }
    pub fn move_right(&mut self, delta_time: f32) {
        let right = Vector3::new(self.yaw.sin(), 0.0, -self.yaw.cos()).normalize();
        self.target += right * self.move_speed * delta_time * self.accesseleration;
        self.update_camera_position();
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera::new(
            Point3::new(5.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, 0.0),
            *Vector3::y_axis(),
            16.0 / 9.0,
            25.0,
            std::f32::consts::FRAC_PI_2,
            0.1,
            100.0,
        )
    }
}

impl fmt::Display for Camera {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Camera {{
            Position: ({:.2}, {:.2}, {:.2}),
            Target: ({:.2}, {:.2}, {:.2}),
            Up: ({:.2}, {:.2}, {:.2}),
            Aspect Ratio: {:.2},
            FOV Y: {:.2} degrees,
            Near: {:.2},
            Far: {:.2},
            Yaw: {:.2} radians,
            Pitch: {:.2} radians,
            Distance: {:.2},
            Is Panning: {},
            Move Speed: {:.2},
            Acceleration: {:.2}
            }}",
            self.position.x,
            self.position.y,
            self.position.z,
            self.target.x,
            self.target.y,
            self.target.z,
            self.up.x,
            self.up.y,
            self.up.z,
            self.aspect_ratio,
            self.fov_y.to_degrees(),
            self.near,
            self.far,
            self.yaw,
            self.pitch,
            self.distance,
            self.is_panning,
            self.move_speed,
            self.accesseleration
        )
    }
}

use std::fmt;

use nalgebra::{Matrix4, Point3, Vector3};
use winit::event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta};
use winit::keyboard::KeyCode;

use crate::input::InputListener;
use crate::log_error;
impl InputListener for Camera {
    fn on_key_event(&mut self, event: &KeyEvent, delta_time: f32) {
        match event.physical_key {
            winit::keyboard::PhysicalKey::Code(KeyCode::KeyW) => self.move_forward(-delta_time),
            winit::keyboard::PhysicalKey::Code(KeyCode::KeyS) => self.move_forward(delta_time),
            winit::keyboard::PhysicalKey::Code(KeyCode::KeyA) => self.move_right(-delta_time),
            winit::keyboard::PhysicalKey::Code(KeyCode::KeyD) => self.move_right(delta_time),
            winit::keyboard::PhysicalKey::Code(KeyCode::ArrowUp) => self.pan(0.0, delta_time),
            winit::keyboard::PhysicalKey::Code(KeyCode::ArrowDown) => self.pan(0.0, -delta_time),
            winit::keyboard::PhysicalKey::Code(KeyCode::ArrowLeft) => self.pan(-delta_time, 0.0),
            winit::keyboard::PhysicalKey::Code(KeyCode::ArrowRight) => self.pan(delta_time, 0.0),
            _ => (),
        }
    }
    fn on_mouse_motion(&mut self, delta: (f64, f64)) {
        if !self.is_panning {
            self.rotate(delta.0 as f32 * 0.01, delta.1 as f32 * 0.01);
        } else {
            let delta_x = delta.0 as f32 * 0.05;
            let delta_y = delta.1 as f32 * 0.05;
            self.pan(delta_x, delta_y);
        }
    }
    fn on_mouse_button(&mut self, button: MouseButton, state: ElementState) {
        if button == MouseButton::Left {
            self.is_panning = state == ElementState::Pressed;
        }
    }
    fn on_scroll(&mut self, delta: MouseScrollDelta) {
        match delta {
            MouseScrollDelta::LineDelta(_, scroll_y) => self.zoom(scroll_y),
            MouseScrollDelta::PixelDelta(delta) => self.zoom(delta.y as f32 * 0.1),
        }
    }
}
#[repr(C)]
#[derive(Default, bytemuck::Pod, bytemuck::Zeroable, Clone, Copy)]
pub struct Plane {
    pub normal: [f32; 3],
    pub distance: f32,
}

#[repr(C)]
#[derive(Default, bytemuck::Pod, bytemuck::Zeroable, Clone, Copy)]
pub struct Frustum {
    pub planes: [Plane; 6],
}

impl Frustum {
    pub fn from_view_projection_matrix(vp_matrix: &Matrix4<f32>) -> Self {
        let mut planes = [
            Self::extract_plane(vp_matrix, 0, 3, 0),  // Left
            Self::extract_plane(vp_matrix, 1, 3, 0),  // Right
            Self::extract_plane(vp_matrix, 2, 3, 0),  // Bottom
            Self::extract_plane(vp_matrix, 3, 3, 0),  // Top
            Self::extract_plane(vp_matrix, 2, 3, -1), // Near
            Self::extract_plane(vp_matrix, 2, 3, 1),  // Far
        ];

        for plane in &mut planes {
            let normal = Vector3::from(plane.normal);
            let length = normal.norm();
            plane.normal = (normal / length).into();
            plane.distance /= length;
        }

        Frustum { planes }
    }

    fn extract_plane(vp_matrix: &Matrix4<f32>, row: usize, _column: usize, sign: i32) -> Plane {
        let m = vp_matrix;

        let normal = Vector3::new(
            m[(0, 3)] + sign as f32 * m[(0, row)],
            m[(1, 3)] + sign as f32 * m[(1, row)],
            m[(2, 3)] + sign as f32 * m[(2, row)],
        );

        let distance = m[(3, 3)] + sign as f32 * m[(3, row)];

        Plane {
            normal: normal.into(),
            distance,
        }
    }

    pub fn contains_sphere(&self, center: Vector3<f32>, radius: f32) -> bool {
        for plane in &self.planes {
            let normal = Vector3::from(plane.normal);
            let distance = normal.dot(&center) + plane.distance;
            if distance < -radius {
                return false;
            }
        }
        true
    }

    pub fn corners(&self) -> [Vector3<f32>; 8] {
        let planes = &self.planes;
        [
            Self::intersect_planes(&planes[0], &planes[2], &planes[4]), // Near bottom-left
            Self::intersect_planes(&planes[1], &planes[2], &planes[4]), // Near bottom-right
            Self::intersect_planes(&planes[1], &planes[3], &planes[4]), // Near top-right
            Self::intersect_planes(&planes[0], &planes[3], &planes[4]), // Near top-left
            Self::intersect_planes(&planes[0], &planes[2], &planes[5]), // Far bottom-left
            Self::intersect_planes(&planes[1], &planes[2], &planes[5]), // Far bottom-right
            Self::intersect_planes(&planes[1], &planes[3], &planes[5]), // Far top-right
            Self::intersect_planes(&planes[0], &planes[3], &planes[5]), // Far top-left
        ]
    }

    fn intersect_planes(p1: &Plane, p2: &Plane, p3: &Plane) -> Vector3<f32> {
        let n1 = Vector3::from(p1.normal);
        let n2 = Vector3::from(p2.normal);
        let n3 = Vector3::from(p3.normal);
        let d1 = p1.distance;
        let d2 = p2.distance;
        let d3 = p3.distance;

        let cross_n2_n3 = n2.cross(&n3);
        let cross_n3_n1 = n3.cross(&n1);
        let cross_n1_n2 = n1.cross(&n2);

        let numerator = (cross_n2_n3 * d1) + (cross_n3_n1 * d2) + (cross_n1_n2 * d3);
        let denominator = n1.dot(&cross_n2_n3);

        if denominator == 0.0 {
            log_error!("Intersection calculation failed due to zero denominator");
            return Vector3::new(0.0, 0.0, 0.0);
        }

        numerator / denominator
    }
}

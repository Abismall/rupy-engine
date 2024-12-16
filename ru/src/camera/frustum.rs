use std::f32::EPSILON;

use cgmath::{InnerSpace, Matrix4, Vector3, Zero};

use super::handler::CameraHandler;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Frustum {
    pub planes: [Plane; 6],
}

impl Frustum {
    pub fn from_camera_handler(handler: &CameraHandler) -> Frustum {
        Frustum::from_view_projection_matrix(handler.view_projection_matrix())
    }

    pub fn from_view_projection_matrix(vp_matrix: Matrix4<f32>) -> Self {
        let planes = Self::extract_planes(vp_matrix);

        Frustum { planes }
    }

    pub fn extract_planes(vp_matrix: Matrix4<f32>) -> [Plane; 6] {
        let vp: [[f32; 4]; 4] = vp_matrix.into();
        let mut planes = [
            Self::extract_plane(&vp, 0, 3, 1),  // Left
            Self::extract_plane(&vp, 0, 3, -1), // Right
            Self::extract_plane(&vp, 1, 3, 1),  // Bottom
            Self::extract_plane(&vp, 1, 3, -1), // Top
            Self::extract_plane(&vp, 2, 3, 1),  // Near
            Self::extract_plane(&vp, 2, 3, -1), // Far
        ];
        for plane in &mut planes {
            plane.normalize();
        }
        planes
    }
    pub fn update_planes(&mut self, vp_matrix: Matrix4<f32>) {
        self.planes = Frustum::extract_planes(vp_matrix);
    }
    fn extract_plane(vp_matrix: &[[f32; 4]; 4], row: usize, column: usize, sign: i32) -> Plane {
        Plane::new(
            Vector3::new(
                vp_matrix[column][3] + sign as f32 * vp_matrix[column][row],
                vp_matrix[1][column] + sign as f32 * vp_matrix[1][row],
                vp_matrix[2][column] + sign as f32 * vp_matrix[2][row],
            ),
            vp_matrix[3][column] + sign as f32 * vp_matrix[3][row],
        )
    }
    pub fn contains_aabb(&self, min: Vector3<f32>, max: Vector3<f32>) -> bool {
        for plane in &self.planes {
            let corners = [
                Vector3::new(min.x, min.y, min.z),
                Vector3::new(max.x, min.y, min.z),
                Vector3::new(min.x, max.y, min.z),
                Vector3::new(max.x, max.y, min.z),
                Vector3::new(min.x, min.y, max.z),
                Vector3::new(max.x, min.y, max.z),
                Vector3::new(min.x, max.y, max.z),
                Vector3::new(max.x, max.y, max.z),
            ];

            if corners
                .iter()
                .all(|corner| plane.distance_to_point(*corner) < 0.0)
            {
                return false;
            }
        }
        true
    }

    pub fn is_in_front_of_camera(
        &self,
        camera_position: Vector3<f32>,
        camera_forward: Vector3<f32>,
        object_position: Vector3<f32>,
    ) -> bool {
        let to_object = object_position - camera_position;
        camera_forward.dot(to_object) > 0.0
    }
    pub fn contains_sphere(&self, center: Vector3<f32>, radius: f32) -> bool {
        for plane in &self.planes {
            if plane.distance_to_point(center) < -radius {
                return false;
            }
        }
        true
    }
    pub fn calculate_instance_radius(scale: Vector3<f32>) -> f32 {
        scale.magnitude() / 2.0
    }

    pub fn contains(&self, volume: &BoundingVolume) -> bool {
        match volume {
            BoundingVolume::Sphere { center, radius } => self.contains_sphere(*center, *radius),
            BoundingVolume::AABB { min, max } => self.contains_aabb(*min, *max),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Plane {
    pub normal: Vector3<f32>,
    pub distance: f32,
}

impl Plane {
    pub fn new(normal: Vector3<f32>, distance: f32) -> Self {
        let mut plane = Plane { normal, distance };
        plane.normalize();
        plane
    }
    pub fn distance_to_point(&self, point: Vector3<f32>) -> f32 {
        self.normal.dot(point) + self.distance
    }
    pub fn normalize(&mut self) {
        let magnitude = self.normal.magnitude();
        if magnitude > 1e-6 {
            self.normal /= magnitude;
            self.distance /= magnitude;
        } else {
            self.normal = Vector3::zero();
            self.distance = 0.0;
        }
    }
}

pub enum BoundingVolume {
    Sphere {
        center: Vector3<f32>,
        radius: f32,
    },
    AABB {
        min: Vector3<f32>,
        max: Vector3<f32>,
    },
}

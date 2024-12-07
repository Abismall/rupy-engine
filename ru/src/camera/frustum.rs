use std::f32::EPSILON;

use cgmath::{InnerSpace, Matrix4, Vector3};

use crate::{log_error, log_info};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Frustum {
    pub planes: [Plane; 6],
}

impl Frustum {
    pub fn from_view_projection_matrix(vp_matrix: Matrix4<f32>) -> Self {
        let vp_array = vp_matrix.into();
        let mut planes = [
            Self::extract_plane(&vp_array, 0, 3, 0),  // Left
            Self::extract_plane(&vp_array, 1, 3, 0),  // Right
            Self::extract_plane(&vp_array, 2, 3, 0),  // Bottom
            Self::extract_plane(&vp_array, 3, 3, 0),  // Top
            Self::extract_plane(&vp_array, 2, 3, -1), // Near
            Self::extract_plane(&vp_array, 2, 3, 1),  // Far
        ];

        for plane in &mut planes {
            plane.normalize();
        }

        Frustum { planes }
    }

    fn extract_plane(vp_matrix: &[[f32; 4]; 4], row: usize, _column: usize, sign: i32) -> Plane {
        Plane {
            normal: Vector3::new(
                vp_matrix[0][3] + sign as f32 * vp_matrix[0][row],
                vp_matrix[1][3] + sign as f32 * vp_matrix[1][row],
                vp_matrix[2][3] + sign as f32 * vp_matrix[2][row],
            ),
            distance: vp_matrix[3][3] + sign as f32 * vp_matrix[3][row],
        }
    }

    pub fn contains_sphere(&self, center: Vector3<f32>, radius: f32) -> bool {
        for plane in &self.planes {
            if plane.distance_to_point(center) < -radius {
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
        let n1 = p1.normal;
        let n2 = p2.normal;
        let n3 = p3.normal;
        let d1 = p1.distance;
        let d2 = p2.distance;
        let d3 = p3.distance;

        let cross_n2_n3 = n2.cross(n3);
        let cross_n3_n1 = n3.cross(n1);
        let cross_n1_n2 = n1.cross(n2);

        let numerator = (cross_n2_n3 * d1) + (cross_n3_n1 * d2) + (cross_n1_n2 * d3);
        let denominator = n1.dot(cross_n2_n3);

        if denominator.abs() < EPSILON {
            log_info!("Denominator near zero: {:?}", denominator);
            log_error!("Intersection calculation failed due to near-zero denominator");
            return Vector3::new(0.0, 0.0, 0.0);
        }

        numerator / denominator
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Plane {
    pub normal: Vector3<f32>,
    pub distance: f32,
}

impl Plane {
    pub fn normalize(&mut self) {
        let magnitude = self.normal.magnitude();
        if magnitude > EPSILON {
            self.normal /= magnitude;
            self.distance /= magnitude;
        }
    }

    pub fn distance_to_point(&self, point: Vector3<f32>) -> f32 {
        self.normal.dot(point) + self.distance
    }
}

use std::f32::EPSILON;

use crate::{ecs::model::Vertex3D, log_error, log_info};

use super::Plane;
use nalgebra::Vector3;
#[repr(C)]
#[derive(Default, bytemuck::Pod, bytemuck::Zeroable, Clone, Copy)]
pub struct Frustum {
    pub planes: [Plane; 6],
}

impl Frustum {
    pub fn from_view_projection_matrix(vp_matrix: &[[f32; 4]; 4]) -> Self {
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

    fn extract_plane(vp_matrix: &[[f32; 4]; 4], row: usize, _column: usize, sign: i32) -> Plane {
        Plane {
            normal: Vector3::new(
                vp_matrix[0][3] + sign as f32 * vp_matrix[0][row],
                vp_matrix[1][3] + sign as f32 * vp_matrix[1][row],
                vp_matrix[2][3] + sign as f32 * vp_matrix[2][row],
            )
            .into(),
            distance: vp_matrix[3][3] + sign as f32 * vp_matrix[3][row],
        }
    }

    pub fn contains_sphere(&self, center: Vector3<f32>, radius: f32) -> bool {
        for plane in &self.planes {
            let distance = Vector3::from(plane.normal).dot(&center) + plane.distance;
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

        if denominator.abs() < EPSILON {
            log_info!("{:?}", denominator.abs());
            log_error!("Intersection calculation failed due to near-zero denominator");
            return Vector3::new(0.0, 0.0, 0.0);
        }

        numerator / denominator
    }
    pub fn generate_plane_geometry(&self) -> Vec<(Vec<Vertex3D>, Vec<u16>)> {
        let corners = self.corners();

        vec![
            // Near plane
            (
                vec![
                    Vertex3D {
                        position: corners[0].into(),
                        color: [1.0, 0.0, 0.0, 1.0],
                        normal: [0.0, 0.0, -1.0],
                        tex_coords: [0.0, 0.0],
                    },
                    Vertex3D {
                        position: corners[1].into(),
                        color: [1.0, 0.0, 0.0, 1.0],
                        normal: [0.0, 0.0, -1.0],
                        tex_coords: [1.0, 0.0],
                    },
                    Vertex3D {
                        position: corners[2].into(),
                        color: [1.0, 0.0, 0.0, 1.0],
                        normal: [0.0, 0.0, -1.0],
                        tex_coords: [1.0, 1.0],
                    },
                    Vertex3D {
                        position: corners[3].into(),
                        color: [1.0, 0.0, 0.0, 1.0],
                        normal: [0.0, 0.0, -1.0],
                        tex_coords: [0.0, 1.0],
                    },
                ],
                vec![0, 1, 2, 2, 3, 0],
            ),
            // Far plane
            (
                vec![
                    Vertex3D {
                        position: corners[4].into(),
                        color: [0.0, 1.0, 0.0, 1.0],
                        normal: [0.0, 0.0, 1.0],
                        tex_coords: [0.0, 0.0],
                    },
                    Vertex3D {
                        position: corners[5].into(),
                        color: [0.0, 1.0, 0.0, 1.0],
                        normal: [0.0, 0.0, 1.0],
                        tex_coords: [1.0, 0.0],
                    },
                    Vertex3D {
                        position: corners[6].into(),
                        color: [0.0, 1.0, 0.0, 1.0],
                        normal: [0.0, 0.0, 1.0],
                        tex_coords: [1.0, 1.0],
                    },
                    Vertex3D {
                        position: corners[7].into(),
                        color: [0.0, 1.0, 0.0, 1.0],
                        normal: [0.0, 0.0, 1.0],
                        tex_coords: [0.0, 1.0],
                    },
                ],
                vec![0, 1, 2, 2, 3, 0],
            ),
            // Left plane
            (
                vec![
                    Vertex3D {
                        position: corners[0].into(),
                        color: [0.0, 0.0, 1.0, 1.0],
                        normal: [-1.0, 0.0, 0.0],
                        tex_coords: [0.0, 0.0],
                    },
                    Vertex3D {
                        position: corners[3].into(),
                        color: [0.0, 0.0, 1.0, 1.0],
                        normal: [-1.0, 0.0, 0.0],
                        tex_coords: [1.0, 0.0],
                    },
                    Vertex3D {
                        position: corners[7].into(),
                        color: [0.0, 0.0, 1.0, 1.0],
                        normal: [-1.0, 0.0, 0.0],
                        tex_coords: [1.0, 1.0],
                    },
                    Vertex3D {
                        position: corners[4].into(),
                        color: [0.0, 0.0, 1.0, 1.0],
                        normal: [-1.0, 0.0, 0.0],
                        tex_coords: [0.0, 1.0],
                    },
                ],
                vec![0, 1, 2, 2, 3, 0],
            ),
            // Right plane
            (
                vec![
                    Vertex3D {
                        position: corners[1].into(),
                        color: [1.0, 1.0, 0.0, 1.0],
                        normal: [1.0, 0.0, 0.0],
                        tex_coords: [0.0, 0.0],
                    },
                    Vertex3D {
                        position: corners[2].into(),
                        color: [1.0, 1.0, 0.0, 1.0],
                        normal: [1.0, 0.0, 0.0],
                        tex_coords: [1.0, 0.0],
                    },
                    Vertex3D {
                        position: corners[6].into(),
                        color: [1.0, 1.0, 0.0, 1.0],
                        normal: [1.0, 0.0, 0.0],
                        tex_coords: [1.0, 1.0],
                    },
                    Vertex3D {
                        position: corners[5].into(),
                        color: [1.0, 1.0, 0.0, 1.0],
                        normal: [1.0, 0.0, 0.0],
                        tex_coords: [0.0, 1.0],
                    },
                ],
                vec![0, 1, 2, 2, 3, 0],
            ),
            // Top plane
            (
                vec![
                    Vertex3D {
                        position: corners[3].into(),
                        color: [1.0, 0.0, 1.0, 1.0],
                        normal: [0.0, 1.0, 0.0],
                        tex_coords: [0.0, 0.0],
                    },
                    Vertex3D {
                        position: corners[2].into(),
                        color: [1.0, 0.0, 1.0, 1.0],
                        normal: [0.0, 1.0, 0.0],
                        tex_coords: [1.0, 0.0],
                    },
                    Vertex3D {
                        position: corners[6].into(),
                        color: [1.0, 0.0, 1.0, 1.0],
                        normal: [0.0, 1.0, 0.0],
                        tex_coords: [1.0, 1.0],
                    },
                    Vertex3D {
                        position: corners[7].into(),
                        color: [1.0, 0.0, 1.0, 1.0],
                        normal: [0.0, 1.0, 0.0],
                        tex_coords: [0.0, 1.0],
                    },
                ],
                vec![0, 1, 2, 2, 3, 0],
            ),
            // Bottom plane
            (
                vec![
                    Vertex3D {
                        position: corners[0].into(),
                        color: [0.0, 1.0, 1.0, 1.0],
                        normal: [0.0, -1.0, 0.0],
                        tex_coords: [0.0, 0.0],
                    },
                    Vertex3D {
                        position: corners[1].into(),
                        color: [0.0, 1.0, 1.0, 1.0],
                        normal: [0.0, -1.0, 0.0],
                        tex_coords: [1.0, 0.0],
                    },
                    Vertex3D {
                        position: corners[5].into(),
                        color: [0.0, 1.0, 1.0, 1.0],
                        normal: [0.0, -1.0, 0.0],
                        tex_coords: [1.0, 1.0],
                    },
                    Vertex3D {
                        position: corners[4].into(),
                        color: [0.0, 1.0, 1.0, 1.0],
                        normal: [0.0, -1.0, 0.0],
                        tex_coords: [0.0, 1.0],
                    },
                ],
                vec![0, 1, 2, 2, 3, 0],
            ),
        ]
    }
}

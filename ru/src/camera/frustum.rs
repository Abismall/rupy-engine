use crate::prelude::{dot_vec4, mat4_mul, vec3_max, vec3_min, Mat4, Vec3, Vec4};

use super::{perspective::CameraPerspective, Camera};

/// Axis-Aligned Bounding Box (AABB)
#[derive(Clone, Debug)]
pub struct AABB {
    pub min: Vec3, // Minimum point of the AABB
    pub max: Vec3, // Maximum point of the AABB
}

impl AABB {
    /// Create a new AABB given its minimum and maximum points
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    /// Create an AABB from a list of vertices
    pub fn from_vertices(vertices: &[Vec3]) -> Self {
        let mut min = vertices[0];
        let mut max = vertices[0];

        for &vertex in vertices {
            min = vec3_min(min, vertex);
            max = vec3_max(max, vertex);
        }

        Self { min, max }
    }

    /// Check if the AABB intersects with a frustum
    pub fn intersects_frustum(&self, frustum: &Frustum) -> bool {
        for plane in &frustum.planes {
            // Find the positive and negative extents of the AABB relative to the plane
            let mut positive_vertex = self.min;
            let mut negative_vertex = self.max;

            // For each axis, choose the farthest positive and negative vertices
            if plane[0] > 0.0 {
                positive_vertex[0] = self.max[0];
                negative_vertex[0] = self.min[0];
            } else {
                positive_vertex[0] = self.min[0];
                negative_vertex[0] = self.max[0];
            }

            if plane[1] > 0.0 {
                positive_vertex[1] = self.max[1];
                negative_vertex[1] = self.min[1];
            } else {
                positive_vertex[1] = self.min[1];
                negative_vertex[1] = self.max[1];
            }

            if plane[2] > 0.0 {
                positive_vertex[2] = self.max[2];
                negative_vertex[2] = self.min[2];
            } else {
                positive_vertex[2] = self.min[2];
                negative_vertex[2] = self.max[2];
            }

            // If the positive vertex is behind the plane, the AABB is outside the frustum
            if dot_vec4(
                *plane,
                [
                    positive_vertex[0],
                    positive_vertex[1],
                    positive_vertex[2],
                    1.0,
                ],
            ) < 0.0
            {
                return false;
            }
        }

        // If the AABB is not fully outside any plane, it is inside or intersecting the frustum
        true
    }
}

pub struct Frustum {
    planes: [Vec4; 6], // 6 planes: near, far, left, right, top, bottom
}

impl Frustum {
    pub fn from_camera(camera: &Camera, perspective: &CameraPerspective) -> Self {
        let view_matrix = camera.view_matrix();
        let proj_matrix = camera.projection_matrix(perspective);
        let view_proj_matrix = mat4_mul(view_matrix, proj_matrix);

        let mut planes = frustum_planes_from_view_projection(view_proj_matrix);
        for plane in &mut planes {
            normalize_plane(plane);
        }

        Self { planes }
    }

    pub fn is_in_frustum(&self, aabb: &AABB) -> bool {
        aabb.intersects_frustum(self)
    }
}

fn frustum_planes_from_view_projection(view_proj: Mat4) -> [Vec4; 6] {
    [
        // Left
        [
            view_proj[3][0] + view_proj[0][0],
            view_proj[3][1] + view_proj[0][1],
            view_proj[3][2] + view_proj[0][2],
            view_proj[3][3] + view_proj[0][3],
        ],
        // Right
        [
            view_proj[3][0] - view_proj[0][0],
            view_proj[3][1] - view_proj[0][1],
            view_proj[3][2] - view_proj[0][2],
            view_proj[3][3] - view_proj[0][3],
        ],
        // Bottom
        [
            view_proj[3][0] + view_proj[1][0],
            view_proj[3][1] + view_proj[1][1],
            view_proj[3][2] + view_proj[1][2],
            view_proj[3][3] + view_proj[1][3],
        ],
        // Top
        [
            view_proj[3][0] - view_proj[1][0],
            view_proj[3][1] - view_proj[1][1],
            view_proj[3][2] - view_proj[1][2],
            view_proj[3][3] - view_proj[1][3],
        ],
        // Near
        [
            view_proj[3][0] + view_proj[2][0],
            view_proj[3][1] + view_proj[2][1],
            view_proj[3][2] + view_proj[2][2],
            view_proj[3][3] + view_proj[2][3],
        ],
        // Far
        [
            view_proj[3][0] - view_proj[2][0],
            view_proj[3][1] - view_proj[2][1],
            view_proj[3][2] - view_proj[2][2],
            view_proj[3][3] - view_proj[2][3],
        ],
    ]
}

fn normalize_plane(plane: &mut Vec4) {
    let normal = [plane[0], plane[1], plane[2]];
    let length = (normal[0].powi(2) + normal[1].powi(2) + normal[2].powi(2)).sqrt();
    for i in 0..4 {
        plane[i] /= length;
    }
}

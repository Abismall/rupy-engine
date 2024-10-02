use super::Vertex;

#[derive(Debug, Clone)]
pub struct Shape {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub is_textured: bool,
}

impl Shape {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, is_textured: bool) -> Self {
        Shape {
            vertices,
            indices,
            is_textured,
        }
    }
}

pub struct ShapeBuilder;

impl ShapeBuilder {
    pub fn build_cube(size: f32) -> Shape {
        let half_size = size / 2.0;
        let vertices = vec![
            Vertex {
                position: [-half_size, -half_size, half_size],
                normal: [0.0, 0.0, 1.0],
                color: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: [half_size, -half_size, half_size],
                normal: [0.0, 0.0, 1.0],
                color: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [half_size, half_size, half_size],
                normal: [0.0, 0.0, 1.0],
                color: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: [-half_size, half_size, half_size],
                normal: [0.0, 0.0, 1.0],
                color: [1.0, 1.0, 0.0],
            },
            Vertex {
                position: [-half_size, -half_size, -half_size],
                normal: [0.0, 0.0, -1.0],
                color: [1.0, 0.0, 1.0],
            },
            Vertex {
                position: [half_size, -half_size, -half_size],
                normal: [0.0, 0.0, -1.0],
                color: [0.0, 1.0, 1.0],
            },
            Vertex {
                position: [half_size, half_size, -half_size],
                normal: [0.0, 0.0, -1.0],
                color: [1.0, 1.0, 1.0],
            },
            Vertex {
                position: [-half_size, half_size, -half_size],
                normal: [0.0, 0.0, -1.0],
                color: [0.0, 0.0, 0.0],
            },
        ];

        let indices = vec![
            0, 1, 2, 2, 3, 0, // Front face
            4, 5, 6, 6, 7, 4, // Back face
            0, 1, 5, 5, 4, 0, // Bottom face
            2, 3, 7, 7, 6, 2, // Top face
            0, 3, 7, 7, 4, 0, // Left face
            1, 2, 6, 6, 5, 1, // Right face
        ];

        Shape::new(vertices, indices, false)
    }

    pub fn build_triangle(width: f32, height: f32) -> Shape {
        let half_width = width / 2.0;
        let vertices = vec![
            Vertex {
                position: [0.0, height, 0.0],
                color: [1.0, 0.0, 0.0],
                normal: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [-half_width, 0.0, 0.0],
                color: [0.0, 1.0, 0.0],
                normal: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [half_width, 0.0, 0.0],
                color: [0.0, 0.0, 1.0],
                normal: [0.0, 1.0, 0.0],
            },
        ];

        let indices = vec![0, 1, 2];

        Shape::new(vertices, indices, false)
    }

    pub fn build_sphere(radius: f32, rings: usize, segments: usize) -> Shape {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for ring in 0..=rings {
            let theta = (ring as f32 / rings as f32) * std::f32::consts::PI;
            for segment in 0..=segments {
                let phi = (segment as f32 / segments as f32) * 2.0 * std::f32::consts::PI;

                let x = radius * theta.sin() * phi.cos();
                let y = radius * theta.cos();
                let z = radius * theta.sin() * phi.sin();

                vertices.push(Vertex {
                    position: [x, y, z],
                    normal: [x, y, z],
                    color: [1.0, 1.0, 1.0],
                });

                if ring > 0 && segment > 0 {
                    let a = ring * (segments + 1) + segment;
                    let b = (ring - 1) * (segments + 1) + segment;
                    let c = (ring - 1) * (segments + 1) + (segment - 1);
                    let d = ring * (segments + 1) + (segment - 1);
                    indices.push(a as u32);
                    indices.push(b as u32);
                    indices.push(c as u32);
                    indices.push(c as u32);
                    indices.push(d as u32);
                    indices.push(a as u32);
                }
            }
        }

        Shape::new(vertices, indices, false)
    }
}

use crate::ecs::model::Vertex3D;

#[derive(Debug, Clone)]
pub struct Plane3D {
    pub vertices: Vec<Vertex3D>,
    pub indices: Vec<u16>,
    pub width: f32,
    pub height: f32,
    pub expansion_rate: f32,
}

impl Plane3D {
    pub fn new(initial_width: f32, initial_height: f32, expansion_rate: f32) -> Self {
        let half_width = initial_width / 2.0;
        let half_height = initial_height / 2.0;

        let vertices = vec![
            Vertex3D {
                position: [-half_width, 0.0, -half_height],
                color: [1.0, 1.0, 1.0, 1.0],
                normal: [0.0, 1.0, 0.0],
                tex_coords: [0.0, 1.0],
            },
            Vertex3D {
                position: [half_width, 0.0, -half_height],
                color: [1.0, 1.0, 1.0, 1.0],
                normal: [0.0, 1.0, 0.0],
                tex_coords: [1.0, 1.0],
            },
            Vertex3D {
                position: [half_width, 0.0, half_height],
                color: [1.0, 1.0, 1.0, 1.0],
                normal: [0.0, 1.0, 0.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex3D {
                position: [-half_width, 0.0, half_height],
                color: [1.0, 1.0, 1.0, 1.0],
                normal: [0.0, 1.0, 0.0],
                tex_coords: [0.0, 0.0],
            },
        ];

        let indices = vec![0, 1, 2, 2, 3, 0];

        Self {
            vertices,
            indices,
            width: initial_width,
            height: initial_height,
            expansion_rate,
        }
    }

    pub fn expand(&mut self, camera_position: [f32; 3]) {
        let distance_from_origin =
            ((camera_position[0].powi(2) + camera_position[2].powi(2)).sqrt() - self.width / 2.0)
                .max(0.0);

        if distance_from_origin > 0.0 {
            let expansion_amount = distance_from_origin * self.expansion_rate;

            self.width += expansion_amount;
            self.height += expansion_amount;

            let half_width = self.width / 2.0;
            let half_height = self.height / 2.0;

            self.vertices = vec![
                Vertex3D {
                    position: [-half_width, 0.0, -half_height],
                    ..self.vertices[0]
                },
                Vertex3D {
                    position: [half_width, 0.0, -half_height],
                    ..self.vertices[1]
                },
                Vertex3D {
                    position: [half_width, 0.0, half_height],
                    ..self.vertices[2]
                },
                Vertex3D {
                    position: [-half_width, 0.0, half_height],
                    ..self.vertices[3]
                },
            ];
        }
    }
}

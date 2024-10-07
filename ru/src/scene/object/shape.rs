use crate::ecs::vertex::Vertex;

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
            is_textured: true,
        }
    }
}

pub enum ShapeSize {
    Small,
    Medium,
    Large,
    Custom(f32),
}

const SIZE_SMALL: f32 = 1.0;
const SIZE_MEDIUM: f32 = 4.0;
const SIZE_LARGE: f32 = 8.0;

impl ShapeSize {
    pub fn as_f32(self) -> f32 {
        match self {
            ShapeSize::Small => SIZE_SMALL,
            ShapeSize::Medium => SIZE_MEDIUM,
            ShapeSize::Large => SIZE_LARGE,
            ShapeSize::Custom(size) => size,
        }
    }
}

struct Cube {
    pub size: f32,
    pub segments: u32,
}

impl Cube {
    pub fn new(size: f32, segments: u32) -> Cube {
        Cube { size, segments }
    }
}

struct Triangle {
    pub size: f32,
}

impl Triangle {
    pub fn new(size: f32) -> Triangle {
        Triangle { size }
    }
}

struct Sphere {
    pub radius: f32,
    pub rings: usize,
    pub segments: usize,
}

impl Sphere {
    pub fn new(radius: f32, rings: usize, segments: usize) -> Sphere {
        Sphere {
            radius,
            rings,
            segments,
        }
    }
}

pub struct ShapeBuilder;

// impl ShapeBuilder {
//     pub fn cube(size: ShapeSize, segments: u32, textured: bool) -> Shape {}

//     pub fn triangle(size: ShapeSize) -> Shape {}

//     pub fn sphere(radius: ShapeSize, rings: usize, segments: usize, textured: bool) -> Shape {}
// }

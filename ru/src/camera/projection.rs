use cgmath::{Matrix4, Rad};

#[derive(Debug)]
pub struct Projection {
    pub aspect: f64,
    pub fovy: Rad<f32>,
    pub znear: f32,
    pub zfar: f32,
}

impl Projection {
    pub fn new<F: Into<Rad<f32>>>(aspect_ratio: f64, fovy: F, znear: f32, zfar: f32) -> Self {
        Self {
            aspect: aspect_ratio,
            fovy: fovy.into(),
            znear,
            zfar,
        }
    }

    pub fn set_aspect_ratio<P: Into<f64>>(&mut self, width: P, height: P) {
        self.aspect = width.into() / height.into();
    }

    pub fn calc_matrix(&self) -> Matrix4<f32> {
        cgmath::perspective(self.fovy, self.aspect as f32, self.znear, self.zfar)
    }
}

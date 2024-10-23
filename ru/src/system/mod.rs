use crate::model::primitive::Primitive;
pub mod audio;
pub mod camera;
pub mod console;
pub mod glyphon;
pub mod input;
pub mod menu;
pub mod plane;
pub mod ui;
pub fn create_ui_boundaries(size_ratio: (f32, f32)) -> (Vec<Primitive>, Vec<u16>) {
    let (width_ratio, height_ratio) = size_ratio;

    let x_min = -1.0;
    let x_max = 1.0;

    let y_min = -1.0 + 2.0 * (1.0 - height_ratio);
    let y_max = 1.0;
    let indices = vec![0, 1, 2, 0, 2, 3];
    let vertices = vec![
        Primitive {
            position: [x_min, y_max, 0.0],
            color: [1.0, 1.0, 1.0, 1.0],
            uv: [0.0, 0.0],
        },
        Primitive {
            position: [x_max, y_max, 0.0],
            color: [1.0, 1.0, 1.0, 1.0],
            uv: [1.0, 0.0],
        },
        Primitive {
            position: [x_max, y_min, 0.0],
            color: [1.0, 1.0, 1.0, 1.0],
            uv: [1.0, 1.0],
        },
        Primitive {
            position: [x_min, y_min, 0.0],
            color: [1.0, 1.0, 1.0, 1.0],
            uv: [0.0, 1.0],
        },
    ];
    (vertices, indices)
}

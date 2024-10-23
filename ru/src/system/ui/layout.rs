use super::UIComponent;

#[derive(Debug)]
pub enum HorizontalAlign {
    Left,
    Center,
    Right,
}

#[derive(Debug)]
pub enum VerticalAlign {
    Top,
    Middle,
    Bottom,
}

#[derive(Debug)]
pub struct UILayout {
    pub components: Vec<UIComponent>,
    pub window_size: (f32, f32),
}

impl UILayout {
    pub fn new(window_size: (f32, f32)) -> Self {
        Self {
            components: Vec::new(),
            window_size,
        }
    }

    pub fn add_component(&mut self, component: UIComponent) {
        self.components.push(component);
    }

    pub fn resize(&mut self, size: &(f32, f32)) {
        self.window_size = *size;
    }
}

use std::fmt;

pub trait GetValue {
    fn get(&self) -> u32;
}

#[derive(Clone, Copy, Debug)]
pub struct Width(u32);

impl Width {
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    pub fn set(&mut self, value: u32) {
        self.0 = value;
    }
}

impl GetValue for Width {
    fn get(&self) -> u32 {
        self.0
    }
}

impl Default for Width {
    fn default() -> Self {
        Self(0)
    }
}

impl From<u32> for Width {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl fmt::Display for Width {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Height(u32);

impl Height {
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    pub fn set(&mut self, value: u32) {
        self.0 = value;
    }
}

impl GetValue for Height {
    fn get(&self) -> u32 {
        self.0
    }
}

impl Default for Height {
    fn default() -> Self {
        Self(0)
    }
}

impl From<u32> for Height {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl fmt::Display for Height {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Depth(u32);

impl Depth {
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    pub fn set(&mut self, value: u32) {
        self.0 = value;
    }
}

impl GetValue for Depth {
    fn get(&self) -> u32 {
        self.0
    }
}

impl Default for Depth {
    fn default() -> Self {
        Self(0)
    }
}

impl From<u32> for Depth {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl fmt::Display for Depth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get())
    }
}

#[derive(Clone, Debug, Default)]
pub struct Size2D {
    width: Width,
    height: Height,
}

impl Size2D {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width: Width::new(width),
            height: Height::new(height),
        }
    }

    pub fn width(&self) -> Width {
        self.width
    }

    pub fn height(&self) -> Height {
        self.height
    }

    pub fn set_width(&mut self, width: u32) {
        self.width.set(width);
    }

    pub fn set_height(&mut self, height: u32) {
        self.height.set(height);
    }
}

impl fmt::Display for Size2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Size2D (Width: {}, Height: {})", self.width, self.height)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Size3D {
    size_2d: Size2D,
    depth: Depth,
}

impl Size3D {
    pub fn new(width: u32, height: u32, depth: u32) -> Self {
        Self {
            size_2d: Size2D::new(width, height),
            depth: Depth::new(depth),
        }
    }

    pub fn width(&self) -> Width {
        self.size_2d.width()
    }

    pub fn height(&self) -> Height {
        self.size_2d.height()
    }

    pub fn depth(&self) -> Depth {
        self.depth
    }

    pub fn set_width(&mut self, width: u32) {
        self.size_2d.set_width(width);
    }

    pub fn set_height(&mut self, height: u32) {
        self.size_2d.set_height(height);
    }

    pub fn set_depth(&mut self, depth: u32) {
        self.depth.set(depth);
    }
}

impl fmt::Display for Size3D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Size3D (Width: {}, Height: {}, Depth: {})",
            self.width(),
            self.height(),
            self.depth()
        )
    }
}

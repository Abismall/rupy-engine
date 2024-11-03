pub mod binding;
pub mod buffer;
pub mod global;
pub mod glyphon;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderMode {
    Flat,
    Depth,
    WireNoDepth,
    WireWithDepth,
}

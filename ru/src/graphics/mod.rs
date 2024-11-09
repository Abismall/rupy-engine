pub mod binding;
pub mod buffer;
pub mod global;
pub mod glyphon;

pub const TRIANGLE_LIST_TEXTURED_PIPELINE_LABEL: &str = "triangle_list";
pub const TRIANGLE_LIST_TEXTURED_DEPTH_VIEW_PIPELINE_LABEL: &str = "triangle_list_depth_view";

pub const LINE_LIST_TEXTURED_PIPELINE_LABEL: &str = "line_list";
pub const LINE_LIST_TEXTURED_DEPTH_VIEW_PIPELINE_LABEL: &str = "line_list_depth_view";

pub const TRIANGLE_LIST_COLORED_PIPELINE_LABEL: &str = "triangle_list";
pub const TRIANGLE_LIST_COLORED_DEPTH_VIEW_PIPELINE_LABEL: &str = "triangle_list_depth_view";

pub const LINE_LIST_COLORED_PIPELINE_LABEL: &str = "line_list";
pub const LINE_LIST_COLORED_DEPTH_VIEW_PIPELINE_LABEL: &str = "line_list_depth_view";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderMode {
    TriangleListNoDepth,
    TriangleListDepthView,
    LineListNoDepth,
    LineListDepthView,
}

impl RenderMode {
    pub fn pipeline_label(&self) -> &str {
        match self {
            RenderMode::TriangleListNoDepth => TRIANGLE_LIST_TEXTURED_PIPELINE_LABEL,
            RenderMode::TriangleListDepthView => TRIANGLE_LIST_TEXTURED_DEPTH_VIEW_PIPELINE_LABEL,
            RenderMode::LineListNoDepth => LINE_LIST_COLORED_PIPELINE_LABEL,
            RenderMode::LineListDepthView => LINE_LIST_COLORED_DEPTH_VIEW_PIPELINE_LABEL,
        }
    }
    pub fn use_depth_stencil(&self) -> bool {
        match self {
            RenderMode::LineListNoDepth | RenderMode::TriangleListNoDepth => false,
            RenderMode::LineListDepthView | RenderMode::TriangleListDepthView => true,
        }
    }
}

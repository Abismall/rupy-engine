use std::hash::Hash;
use std::hash::Hasher;
#[derive(Clone, Debug)]
pub struct PipelineCacheKey {
    pub shader_path: String,
    pub vertex_entry_point: String,
    pub fragment_entry_point: String,
    pub bind_group_layout_labels: Vec<String>,
    pub topology: wgpu::PrimitiveTopology,
    pub front_face: wgpu::FrontFace,
    pub cull_mode: Option<wgpu::Face>,
    pub polygon_mode: wgpu::PolygonMode,
}

impl PartialEq for PipelineCacheKey {
    fn eq(&self, other: &Self) -> bool {
        self.shader_path == other.shader_path
            && self.vertex_entry_point == other.vertex_entry_point
            && self.fragment_entry_point == other.fragment_entry_point
            && self.bind_group_layout_labels == other.bind_group_layout_labels
            && self.topology == other.topology
            && self.front_face == other.front_face
            && self.cull_mode == other.cull_mode
            && self.polygon_mode == other.polygon_mode
    }
}

impl Eq for PipelineCacheKey {}

impl Hash for PipelineCacheKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.shader_path.hash(state);
        self.vertex_entry_point.hash(state);
        self.fragment_entry_point.hash(state);
        self.bind_group_layout_labels.hash(state);
        self.topology.hash(state);
        self.front_face.hash(state);
        self.cull_mode.hash(state);
        self.polygon_mode.hash(state);
    }
}

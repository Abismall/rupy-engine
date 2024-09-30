pub trait Renderable {
    fn vertex_buffer_data(&self) -> &[u8];

    fn index_buffer_data(&self) -> &[u32];

    fn num_indices(&self) -> u32;

    fn is_textured(&self) -> bool;

    fn update(&mut self);
}

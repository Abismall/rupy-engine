use std::fmt::Debug;

#[derive(Clone, Copy, Debug)]
pub struct MenuItem<T, L> {
    pub label: L,
    pub action: T,
}

impl<T, L> MenuItem<T, L>
where
    L: Debug,
{
    pub fn new(label: L, action: T) -> Self {
        Self { label, action }
    }
}

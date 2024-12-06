use serde::{Deserialize, Serialize};

// pub mod cube;
// pub mod hexagon;
// pub mod plane;
// pub mod rectangle;
// pub mod sphere;
// pub mod triangle;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum GeometryDimension {
    D2,
    D3,
}
#[derive(Debug, Clone, Deserialize)]
pub enum GeometryId {
    Triangle,
    Cube,
}
impl Serialize for GeometryId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        Ok(match self {
            GeometryId::Triangle => serializer.collect_str("Triangle")?,
            GeometryId::Cube => serializer.collect_str("Cube")?,
        })
    }
}

use crate::object::DrawingObject;

#[derive(Deserialize)]
pub struct Raster {
    pub width: i32,
    pub height: i32,
    pub epsilon: f64,
    pub objects: Vec<DrawingObject>,
}

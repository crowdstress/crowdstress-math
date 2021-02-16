use crate::primitives::Point;

#[derive(Serialize, Deserialize)]
pub struct DrawingObject {
    pub id: String,
    pub object_type: u8,
    pub points: Vec<Point>,
}

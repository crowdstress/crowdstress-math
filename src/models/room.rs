use crate::primitives::Polygon;

#[derive(Serialize)]
pub struct Room {
    pub id: String,
    pub points: Polygon,
    pub exits: Vec<String>,
}

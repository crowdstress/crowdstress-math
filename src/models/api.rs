use crowdstress_common::drawing_object::DrawingObject;

#[derive(Deserialize)]
pub struct GetWalls {
    pub objects: Vec<DrawingObject>,
}

#[derive(Deserialize)]
pub struct GetRooms {
    pub width: i32,
    pub height: i32,
    pub epsilon: f64,
    pub objects: Vec<DrawingObject>,
}

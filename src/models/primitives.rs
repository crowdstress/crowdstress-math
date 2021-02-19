#[derive(Clone, Copy)]
pub struct Section {
    pub start: Point,
    pub end: Point,
}

impl Section {
    pub fn new(point1: Point, point2: Point) -> Section {
        Section {
            start: point1,
            end: point2,
        }
    }
}

pub type Polygon = Vec<Point>;

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    pub fn to_i32(&self) -> Point32 {
        Point32 {
            x: self.x as i32,
            y: self.y as i32,
        }
    }
}

pub struct Point32 {
    pub x: i32,
    pub y: i32,
}

impl Point32 {
    pub fn new(x: i32, y: i32) -> Point32 {
        Point32 { x, y }
    }
}

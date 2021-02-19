use crate::object::DrawingObject;
use crate::primitives::{Point, Section};

#[derive(Serialize, Deserialize)]
pub struct Wall {
    pub start: Point,
    pub end: Point,
}

impl Wall {
    pub fn to_line(&self) -> Section {
        Section {
            start: self.start,
            end: self.end,
        }
    }

    pub fn from_object(object: &DrawingObject) -> Vec<Wall> {
        let mut walls: Vec<Wall> = Vec::new();

        if object.object_type == 0 {
            walls.push(Wall {
                start: object.points[0],
                end: object.points[1],
            });
        } else if object.object_type == 1 {
            let mut rect_walls: Vec<Wall> = Vec::with_capacity(4);
            rect_walls.push(Wall {
                start: object.points[0],
                end: Point::new(object.points[1].y, object.points[0].y),
            });
            rect_walls.push(Wall {
                start: Point::new(object.points[1].x, object.points[0].y),
                end: Point::new(object.points[1].x, object.points[1].y),
            });
            rect_walls.push(Wall {
                start: Point::new(object.points[1].x, object.points[1].y),
                end: Point::new(object.points[0].x, object.points[1].y),
            });
            rect_walls.push(Wall {
                start: Point::new(object.points[0].x, object.points[1].y),
                end: object.points[0],
            });
            for rect_wall in rect_walls {
                walls.push(rect_wall);
            }
        }

        walls
    }
}

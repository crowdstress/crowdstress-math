use crate::object::DrawingObject;
use crate::primitives::{Point, Section};

#[derive(Serialize, Deserialize)]
pub struct Wall {
    pub start: Point,
    pub end: Point,
}

impl Wall {
    pub fn to_line(&self) -> Section {
        [self.start, self.end]
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
                end: [object.points[1][0], object.points[0][1]],
            });
            rect_walls.push(Wall {
                start: [object.points[1][0], object.points[0][1]],
                end: [object.points[1][0], object.points[1][1]],
            });
            rect_walls.push(Wall {
                start: [object.points[1][0], object.points[1][1]],
                end: [object.points[0][0], object.points[1][1]],
            });
            rect_walls.push(Wall {
                start: [object.points[0][0], object.points[1][1]],
                end: object.points[0],
            });
            for rect_wall in rect_walls {
                walls.push(rect_wall);
            }
        }

        walls
    }
}

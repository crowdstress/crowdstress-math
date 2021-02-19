use crate::primitives::{Point, Polygon, Section};
use crate::vector::Vector;

pub fn get_section_middle(section: &Section) -> Point {
    Point {
        x: (section.start.x + section.end.x) / 2.0,
        y: (section.start.y + section.end.y) / 2.0,
    }
}

pub fn get_vector_to_line(line: Section, point: Point) -> Vector {
    let n = Vector::from_points(&line.start, &line.end).normalize();
    let a = Vector::new(line.start);
    let p = Vector::new(point);
    let p2a = a.subtract(&p);
    let projection = p2a.dot(&n);
    let projection_vector = n.product(projection);
    if p2a.is_equal_to(&projection_vector) {
        Vector::origin()
    } else {
        p2a.subtract(&projection_vector)
    }
}

pub fn polygon_to_sections(points: &Polygon) -> Vec<Section> {
    let mut sections: Vec<Section> = Vec::with_capacity(points.len());
    for i in 0..points.len() - 1 {
        let polygon_point1 = points[i];
        let polygon_point2 = points[i + 1];
        let section = Section {
            start: polygon_point1,
            end: polygon_point2,
        };
        sections.push(section);
    }
    let closing_section = Section {
        start: points[points.len() - 1],
        end: points[0],
    };
    sections.push(closing_section);

    sections
}

pub fn is_lines_intersects(line1: Section, line2: Section) -> bool {
    let vector1 = (line2.end.x - line2.start.x) * (line1.start.y - line2.start.y)
        - (line2.end.y - line2.start.y) * (line1.start.x - line2.start.x);
    let vector2 = (line2.end.x - line2.start.x) * (line1.end.y - line2.start.y)
        - (line2.end.y - line2.start.y) * (line1.end.x - line2.start.x);
    let vector3 = (line1.end.x - line1.start.x) * (line2.start.y - line1.start.y)
        - (line1.end.y - line1.start.y) * (line2.start.x - line1.start.x);
    let vector4 = (line1.end.x - line1.start.x) * (line2.end.y - line1.start.y)
        - (line1.end.y - line1.start.y) * (line2.end.x - line1.start.x);
    vector1 * vector2 <= 0.0 && vector3 * vector4 <= 0.0
}

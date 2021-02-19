use crate::primitives::{Point, Section};

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn origin() -> Vector {
        Vector { x: 0.0, y: 0.0 }
    }

    pub fn new(point: Point) -> Vector {
        Vector {
            x: point.x,
            y: point.y,
        }
    }

    pub fn from_points(start: &Point, end: &Point) -> Vector {
        let dx = end.x - start.x;
        let dy = end.y - start.y;
        Vector::new(Point::new(dx, dy))
    }

    pub fn to_line(&self, from: Point) -> Section {
        let dx = from.x + self.x;
        let dy = from.y - self.y;
        Section {
            start: from,
            end: Point::new(dx, dy),
        }
    }

    pub fn get_length(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }

    pub fn add(&self, vector: &Vector) -> Vector {
        add(self, vector)
    }

    pub fn divide(&self, n: f64) -> Vector {
        divide(self, n)
    }

    pub fn dot(&self, vector: &Vector) -> f64 {
        dot(self, vector)
    }

    pub fn is_collinear_with(&self, vector: &Vector) -> bool {
        is_collinear(self, vector)
    }

    pub fn is_equal_to(&self, vector: &Vector) -> bool {
        is_equal(self, vector)
    }

    pub fn normalize(&self) -> Vector {
        normalize(self)
    }

    pub fn perpendicular(&self) -> Vector {
        perpendicular(self)
    }

    pub fn product(&self, n: f64) -> Vector {
        product(self, n)
    }

    pub fn projection_to(&self, vector: &Vector) -> f64 {
        projection(self, vector)
    }

    pub fn scalar(&self, vector: &Vector) -> f64 {
        scalar(self, vector)
    }

    pub fn subtract(&self, vector: &Vector) -> Vector {
        subtract(self, vector)
    }
}

fn add(a: &Vector, b: &Vector) -> Vector {
    let x = a.x + b.x;
    let y = a.y + b.y;
    Vector::new(Point::new(x, y))
}

fn angle(a: &Vector, b: &Vector) -> f64 {
    if a.get_length() * b.get_length() == 0.0 {
        0 as f64
    } else {
        (Vector::dot(a, b) / (a.get_length() * b.get_length())).acos()
    }
}

fn divide(a: &Vector, n: f64) -> Vector {
    let x = a.x / n;
    let y = a.y / n;
    Vector::new(Point::new(x, y))
}

fn dot(a: &Vector, b: &Vector) -> f64 {
    a.x * b.x + a.y * b.y
}

fn is_collinear(a: &Vector, b: &Vector) -> bool {
    a.x * b.y - a.y * b.x == 0.0
}

fn is_equal(a: &Vector, b: &Vector) -> bool {
    if a.x == b.x && a.y == b.y {
        true
    } else {
        false
    }
}

fn normalize(a: &Vector) -> Vector {
    if a.get_length() == 0.0 {
        Vector::origin()
    } else {
        let x = a.x / a.get_length();
        let y = a.y / a.get_length();
        Vector::new(Point::new(x, y))
    }
}

fn perpendicular(vector: &Vector) -> Vector {
    Vector::new(Point::new(-vector.y, vector.x))
}

fn product(a: &Vector, n: f64) -> Vector {
    let x = a.x * n;
    let y = a.y * n;
    Vector::new(Point::new(x, y))
}

fn projection(a: &Vector, b: &Vector) -> f64 {
    scalar(a, b) / b.get_length()
}

fn scalar(a: &Vector, b: &Vector) -> f64 {
    a.get_length() * b.get_length() / angle(a, b).cos()
}

fn subtract(a: &Vector, b: &Vector) -> Vector {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    Vector::new(Point::new(dx, dy))
}

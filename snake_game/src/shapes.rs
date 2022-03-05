use std::ops;

#[derive(Copy, Clone)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

#[derive(Copy, Clone)]
pub struct Rect {
    pub pos: Vector2,
    pub dim: Vector2,
}

/*
#[derive(Clone, Copy)]
pub struct Circ {
    pub pos: Vector2,
    pub r: f64
}*/

impl Vector2 {
    pub fn dist(self, p: Vector2) -> f64 {
        (self.square_dist(p)).sqrt()
    }
    pub fn square_dist(self, p: Vector2) -> f64 {
        (p.x - self.x)*(p.x - self.x) + (p.y - self.y)*(p.y - self.y)
    }
}

impl Rect {

    pub fn contains(&self, p: Vector2) -> bool {
        p.x >= self.pos.x &&
        p.x < self.pos.x + self.dim.x &&
        p.y >= self.pos.y &&
        p.y < self.pos.y + self.dim.y
    }
    pub fn intersects(&self, r: Rect) -> bool {
        self.pos.x < r.pos.x + r.dim.x &&
        self.pos.x + self.dim.x > r.pos.x &&
        self.pos.y < r.pos.y + r.dim.y &&
        self.pos.y + self.dim.y > r.pos.y
    }
}
/*
impl Circ {
    pub fn contains(&self, p: Vector2) -> bool {
        let dx = (p.x - self.pos.x).abs();
        let dy = (p.y - self.pos.y).abs();

        if dx >= self.r ||
           dy >= self.r {
               return false
           }

        if dx + dy < self.r {
            return true;
        }

        p.square_dist(self.pos) < self.r*self.r
    }
}
*/
impl ops::Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, other: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl ops::AddAssign<Vector2> for Vector2 {
    fn add_assign(&mut self, other: Vector2) {
        *self = Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

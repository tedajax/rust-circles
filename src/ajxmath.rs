#[deriving(PartialEq, Clone, Show)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[allow(dead_code)]
impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x: x, y: y }
    }

    pub fn zero() -> Vec2 {
        Vec2 { x: 0_f32, y: 0_f32 }
    }

    pub fn one() -> Vec2 {
        Vec2 { x: 1_f32, y: 1_f32 }
    }

    pub fn unit_x() -> Vec2 {
        Vec2 { x: 1_f32, y: 0_f32 }
    }

    pub fn unit_y() -> Vec2 {
        Vec2 { x: 0_f32, y: 1_f32 }
    }

    pub fn mag(&self) -> f32 {
        self.length()
    }

    pub fn magSqr(&self) -> f32 {
        self.length_sqr()
    }

    pub fn length(&self) -> f32 {
        self.length_sqr().sqrt()
    }

    pub fn length_sqr(&self) -> f32 {
        (self.x * self.x) + (self.y * self.y)
    }

    pub fn normalize(&self) -> Vec2 {
        let l = self.length();
        Vec2 {
            x: self.x / l,
            y: self.y / l,
        }
    }

    pub fn dot(&self, _rhs: &Vec2) -> f32 {
        self.x * _rhs.x + self.y * _rhs.y
    }

    pub fn angle(&self, _rhs: &Vec2) -> f32 {
        (self.dot(_rhs) / self.length() * _rhs.length()).acos()
    }

    pub fn distance(v1: Vec2, v2: Vec2) -> f32 {
        Vec2::distance_sqr(v1, v2).sqrt()
    }

    pub fn distance_sqr(v1: Vec2, v2: Vec2) -> f32 {
        (v2.x - v1.x)*(v2.x - v1.x) + (v2.y - v1.y)*(v2.y - v1.y)
    }
}

impl Add<Vec2, Vec2> for Vec2 {
    fn add(&self, _rhs: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl Sub<Vec2, Vec2> for Vec2 {
    fn sub(&self, _rhs: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
        }
    }
}

impl Mul<f32, Vec2> for Vec2 {
    fn mul(&self, _rhs: &f32) -> Vec2 {
        Vec2 {
            x: self.x * (*_rhs),
            y: self.y * (*_rhs),
        }
    }
}

impl Neg<Vec2> for Vec2 {
    fn neg(&self) -> Vec2 {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[deriving(Show, Clone, PartialEq)]
pub struct Rect {
    pub position: Vec2,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn new(pos: Vec2, w: f32, h: f32) -> Rect {
        Rect {
            position: pos,
            width: w,
            height: h,
        }
    }

    pub fn left(&self) -> f32 {
        self.position.x
    }

    pub fn right(&self) -> f32 {
        self.position.x + self.width
    }

    pub fn top(&self) -> f32 {
        self.position.y
    }

    pub fn bottom(&self) -> f32 {
        self.position.y + self.height
    }

    pub fn contains_point(&self, point: Vec2) -> bool {
        point.x >= self.left() &&
        point.x <= self.right() &&
        point.y >= self.top() &&
        point.y <= self.bottom()
    }

    pub fn intersects(&self, other: Rect) -> bool {
        self.right() >= other.left() && self.left() <= other.right() &&
        self.bottom() >= other.top() && self.top() <= other.bottom()
    }
}

#[deriving(Show, Clone, PartialEq)]
pub struct Circle {
    pub position: Vec2,
    pub radius: f32
}

impl Circle {
    pub fn new(pos: Vec2, r: f32) -> Circle {
        Circle {
            position: pos,
            radius: r,
        }
    }


}
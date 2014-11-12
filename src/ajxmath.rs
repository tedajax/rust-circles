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
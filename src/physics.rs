use ajxmath::Vec2;

pub struct Body {
    id: int,
    shape: Shape,
    position: Vec2,
}

pub enum Shape {
    Circle,
    Rectangle,
}

impl Body {
    pub fn next_circle(id: int, pos: Vec2) -> Body {
        Body {
            id: id,
            position: pos,
            shape: Circle,
        }
    }
}

pub struct World {
    objects: Vec<Body>,
    currentId: int,
    gravity: Vec2,
}

impl World {
    pub fn new() -> World {
        World {
            objects: Vec::new(),
            currentId: 1,
            gravity: Vec2::new(0_f32, -9.8_f32),
        }
    }

    pub fn next_id(&mut self) -> int {
        let id = self.currentId;
        self.currentId += 1;
        return id;
    }

    pub fn add_circle(&mut self, pos: Vec2, radius: f32) {
        let id = self.next_id();
        self.objects.push(Body::next_circle(id, pos))
    }
}


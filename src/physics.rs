use ajxmath::Vec2;

pub struct Body {
    pub id: int,
    pub position: Vec2,
    pub velocity: Vec2,
    pub rotation: f32,
    pub angular_velocity: f32,
    pub shape: Shape,
    pub is_static: bool,
}

pub enum Shape {
    None,
    Circle(f32), //radius
    Rectangle(f32, f32), //width, height
    Line(f32), //length (body position is midpoint)
}

impl Body {
    pub fn new(id: int, pos: Vec2) -> Body {
        Body {
            id: id,
            position: pos,
            velocity: Vec2::zero(),
            rotation: 0_f32,
            angular_velocity: 0_f32,
            is_static: false,
            shape: None,
        }
    }

    pub fn add_shape(&mut self, shape: Shape) {
        self.shape = shape;
    }
}

pub struct World {
    pub objects: Vec<Body>,
    current_id: int,
    px_to_meters: int,
    gravity: Vec2,
}

impl World {
    pub fn new(px_to_meters: int) -> World {

        World {
            objects: Vec::new(),
            current_id: 1,
            px_to_meters: px_to_meters,
            gravity: Vec2::new(0_f32, 1000_f32 * px_to_meters as f32),
        }
    }

    pub fn next_id(&mut self) -> int {
        let id = self.current_id;
        self.current_id += 1;
        return id;
    }

    pub fn add_body(&mut self, pos: Vec2, shape: Shape, is_static: bool) {
        let id = self.next_id();
        let mut body = Body::new(id, pos);
        body.add_shape(shape);
        body.is_static = is_static;
        self.objects.push(body);
    }

    pub fn get_body(&mut self, id: int) -> Option<&mut Body> {
        self.objects.iter_mut().filter(|o| o.id == id).nth(0)
    }

    pub fn update(&mut self, dt: f32) {
        for mut body in self.objects.iter_mut() {
            if (body.is_static) {
                continue;
            }

            body.velocity = body.velocity + self.gravity * dt;
            body.position = body.position + body.velocity * dt;
        }
    }
}


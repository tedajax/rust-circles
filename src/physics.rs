use ajxmath::{Vec2, Rect, Circle};

pub struct Body {
    pub id: int,
    pub position: Vec2,
    pub velocity: Vec2,
    pub rotation: f32,
    pub angular_velocity: f32,
    pub mass: f32,
    pub friction: f32,
    pub restitution: f32,
    pub shape: Shape,
    pub is_static: bool,
}

pub enum Shape {
    None,
    ShapeCircle(Circle), //radius
    ShapeRectangle(Rect), //width, height
    // ShapeLine(Rect), //length (body position is midpoint)
}

impl Body {
    pub fn new(id: int, pos: Vec2) -> Body {
        Body {
            id: id,
            position: pos,
            velocity: Vec2::new(500f32, 0f32),
            rotation: 0_f32,
            angular_velocity: 0_f32,
            mass: 1_f32,
            friction: 0_f32,
            restitution: 0.75_f32,
            is_static: false,
            shape: None,
        }
    }

    pub fn add_shape(&mut self, shape: Shape) {
        self.shape = shape;
    }

    pub fn get_bounds(&self) -> Rect {
        match self.shape {
            ShapeCircle(c) => {
                let r = c.radius;
                let d = r * 2_f32 + 1_f32;
                let tl = Vec2::new(c.position.x + self.position.x - r, c.position.y + self.position.y - r);
                Rect::new(tl, d, d)
            },
            ShapeRectangle(r) => {
                Rect::new(self.position + r.position, r.width, r.height)
            }
            _ => Rect::new(Vec2::zero(), 0_f32, 0_f32),
        }
    }

    pub fn collides(&self, other: &Body) -> bool {
        match self.shape {
            ShapeCircle(sc1) => {
                match other.shape {
                    ShapeCircle(sc2) => {
                        let mut c1 = sc1.clone();
                        c1.position = c1.position + self.position;
                        let mut c2 = sc2.clone();
                        c2.position = c2.position + other.position;
                        Body::circle_circle_collision(c1, c2)
                    },
                    ShapeRectangle(sr2) => {
                        let mut c1 = sc1.clone();
                        c1.position = c1.position + self.position;
                        let mut r2 = sr2.clone();
                        r2.position = r2.position + other.position;
                        Body::circle_rectangle_collision(c1, r2)
                    },
                    _ => false
                }
            },
            ShapeRectangle(sr1) => {
                match other.shape {
                    ShapeCircle(sc2) => {
                        let mut r1 = sr1.clone();
                        r1.position = r1.position + self.position;
                        let mut c2 = sc2.clone();
                        c2.position = c2.position + other.position;
                        Body::circle_rectangle_collision(c2, r1)
                    },
                    ShapeRectangle(sr2) => {
                        let mut r1 = sr1.clone();
                        r1.position = r1.position + self.position;
                        let mut r2 = sr2.clone();
                        r2.position = r2.position + other.position;
                        Body::rectangle_rectangle_collision(r1, r2)
                    },
                    _ => false
                }
            },
            _ => false
        }
    }

    fn circle_circle_collision(c1: Circle, c2: Circle) -> bool {
        let d = Vec2::distance(c1.position, c2.position);
        d <= (c1.radius + c2.radius)
    }

    fn circle_rectangle_collision(c1: Circle, r1: Rect) -> bool {
        c1.position.x >= r1.left() - c1.radius &&
        c1.position.x <= r1.right() + c1.radius &&
        c1.position.y >= r1.top() - c1.radius &&
        c1.position.y <= r1.bottom() + c1.radius
    }

    fn rectangle_rectangle_collision(r1: Rect, r2: Rect) -> bool {
        r1.intersects(r2)
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
            gravity: Vec2::new(0_f32, 9.8_f32 * px_to_meters as f32),
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

            if (body.get_bounds().bottom() > 600_f32) {
                body.position.y = 600_f32 - body.get_bounds().height / 2_f32;
                body.velocity.y = -(body.velocity.y * body.restitution);
            }

            if (body.get_bounds().right() > 800_f32) {
                body.position.x = 800_f32 - body.get_bounds().width / 2_f32;
                body.velocity.x = -(body.velocity.x * body.restitution);
            }

            if (body.get_bounds().left() < 0_f32) {
                body.position.x = 0_f32 + body.get_bounds().width / 2_f32;
                body.velocity.x = -(body.velocity.x * body.restitution);
            }
        }

        let len = self.objects.len();
        for i in range(0, len - 1) {
            for j in range(i + 1, len) {
                let obj_slices = self.objects.split_at_mut(j);
                let mut o1;
                let mut o2;

                match obj_slices {
                    (slice1, slice2) => {
                        o1 = slice1[j - 1];
                        o2 = slice2[0];
                    }
                }

                if o1.collides(&o2) {
                    let normal = (o2.position - o1.position).normalize();

                    println!("collide");
                    o1.velocity.x = 0f32;
                }
            }
        }
    }
}


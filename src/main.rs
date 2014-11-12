extern crate sdl2;
extern crate sdl2_gfx;
extern crate native;

use sdl2_gfx::primitives::DrawRenderer;

use ajxmath::Vec2;

mod ajxmath;
mod physics;
mod time;

fn render_body(&body: &physics::Body, &ref renderer: &sdl2::render::Renderer) {
    let pos = body.position;
    let x = pos.x as i16;
    let y = pos.y as i16;
    match body.shape {
        physics::Circle(radius) => {
            let _ = renderer.circle(x, y, radius as i16, sdl2::pixels::RGB(255, 255, 0));
        },
        physics::Rectangle(width, height) => {
            let w = width as i16;
            let h = height as i16;
            let _ = renderer.rectangle(x, y, x + w, y + h, sdl2::pixels::RGB(0, 255, 255));
        },
        physics::Line(length) => {
            let l = (length / 2_f32) as i16;
            let _ = renderer.line(x - l, y, x + l, y, sdl2::pixels::RGB(0, 255, 0));
        },
        _ => {}
    }
}

pub fn main() {
    sdl2::init(sdl2::INIT_VIDEO);

    let window = match sdl2::video::Window::new("Circles", sdl2::video::PosCentered, sdl2::video::PosCentered, 800, 600, sdl2::video::OPENGL) {
        Ok(window) => window,
        Err(err) => panic!(format!("failed to create window: {}", err))
    };

    let renderer = match sdl2::render::Renderer::from_window(window, sdl2::render::DriverAuto, sdl2::render::ACCELERATED) {
        Ok(renderer) => renderer,
        Err(err) => panic!(format!("failed to create renderer: {}", err))
    };

    let mut world = physics::World::new(64);
    world.add_body(Vec2::new(200_f32, 100_f32), physics::Circle(32_f32), false);
    world.add_body(Vec2::new(300_f32, 400_f32), physics::Rectangle(100_f32, 50_f32), true);
    world.add_body(Vec2::new(400_f32, 300_f32), physics::Line(50_f32), true);

    let mut time = time::Time::new();
    let mut frames: int = 0;

    'main : loop {
        'event : loop {
            match sdl2::event::poll_event() {
                sdl2::event::QuitEvent(_) => break 'main,
                sdl2::event::KeyDownEvent(_, _, key, _, _) => {
                    if key == sdl2::keycode::EscapeKey {
                        break 'main
                    }
                },
                sdl2::event::NoEvent => break 'event,
                _ => {}
            }
        }

        time.update();

        let _ = renderer.set_draw_color(sdl2::pixels::RGB(32, 32, 32));
        let _ = renderer.clear();

        //let _ = renderer.circle(pos.x as i16, pos.y as i16, 25_i16, sdl2::pixels::RGB(255, 255, 0));

        let dt = time.delta();
        if (time.second_elapsed()) {
            println!("fps: {}", frames);
            frames = 0;
        } else {
            frames += 1;
        }

        world.update(dt);

        for body in world.objects.iter() {
            render_body(body, &renderer);
        }

        renderer.present();
    }

    sdl2::quit();
}
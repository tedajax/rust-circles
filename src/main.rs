extern crate sdl2;
extern crate sdl2_gfx;
extern crate native;

use sdl2_gfx::primitives::DrawRenderer;

use ajxmath::Vec2;

mod ajxmath;

pub fn main() {
    sdl2::init(sdl2::INIT_VIDEO);

    let window = match sdl2::video::Window::new("rust-sdl2 demo: Video", sdl2::video::PosCentered, sdl2::video::PosCentered, 800, 600, sdl2::video::OPENGL) {
        Ok(window) => window,
        Err(err) => panic!(format!("failed to create window: {}", err))
    };

    let renderer = match sdl2::render::Renderer::from_window(window, sdl2::render::DriverAuto, sdl2::render::ACCELERATED) {
        Ok(renderer) => renderer,
        Err(err) => panic!(format!("failed to create renderer: {}", err))
    };

    let mut pos = Vec2::new(0_f32, 0_f32);

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

        let _ = renderer.set_draw_color(sdl2::pixels::RGB(32, 32, 32));
        let _ = renderer.clear();

        let x = pos.x as i16;
        let y = pos.y as i16;
        let _ = renderer.circle(x, y, 25_i16, sdl2::pixels::RGB(255, 255, 0));
        pos = pos + Vec2::one();

        let _ = renderer.line(0, 0, 100, 50, sdl2::pixels::RGB(255, 0, 0));

        renderer.present();
    }

    sdl2::quit();
}
extern crate piston;
extern crate graphics;
extern crate glium_graphics;

use std::path::Path;
use piston::window::{ WindowSettings, Size };
use piston::input::RenderEvent;
use piston::event_loop::EventLoop;
use glium_graphics::{ Glium2d, GliumWindow, GlyphCache, OpenGL, TextureSettings };

fn main() {
    let opengl = OpenGL::V3_2;
    let size = Size { width: 500, height: 300 };
    let ref mut window: GliumWindow =
        WindowSettings::new("gfx_graphics: text_test", size)
        .exit_on_esc(true).opengl(opengl).build().unwrap();

    let mut glyph_cache = GlyphCache::new(
        Path::new("assets/FiraSans-Regular.ttf"),
        window.clone(),
        TextureSettings::new()
    ).unwrap();

    let mut g2d = Glium2d::new(opengl, window);
    window.set_lazy(true);
    while let Some(e) = window.next() {
        if let Some(args) = e.render_args() {
            let mut target = window.draw();
            g2d.draw(&mut target, args.viewport(), |c, g| {
                use graphics::*;

                clear([1.0; 4], g);
                text::Text::new_color([0.0, 0.5, 0.0, 1.0], 32).draw(
                    "Hello glium_graphics!",
                    &mut glyph_cache,
                    &DrawState::default(),
                    c.transform.trans(10.0, 100.0),
                    g
                ).unwrap();
            });
            target.finish().unwrap();
        }
    }
}

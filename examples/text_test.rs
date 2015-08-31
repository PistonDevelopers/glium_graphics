extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate glium_graphics;

use std::cell::RefCell;
use std::rc::Rc;
use std::path::Path;
use piston::window::{ WindowSettings, Size };
use piston::event_loop::*;
use piston::input::RenderEvent;
use glium_graphics::{ GliumGraphics, Glium2d, GliumWindow, GlyphCache };
use glutin_window::{ GlutinWindow, OpenGL };

fn main() {
    let opengl = OpenGL::V3_2;
    let size = Size { width: 500, height: 300 };
    let ref window: Rc<RefCell<GlutinWindow>> = Rc::new(RefCell::new(
        WindowSettings::new("gfx_graphics: text_test", size)
        .exit_on_esc(true).opengl(opengl).build().unwrap()
    ));

    let ref glium_window = GliumWindow::new(window).unwrap();
    let mut glyph_cache = GlyphCache::new(
        Path::new("assets/FiraSans-Regular.ttf"),
        glium_window.clone()
    ).unwrap();

    let mut g2d = Glium2d::new(opengl, glium_window);

    for _ in window.events().swap_buffers(false)
        .filter_map(|event| event.render_args())
    {    
        let mut target = glium_window.draw();
        {
            use graphics::*;
            let mut g = GliumGraphics::new(&mut g2d, &mut target);
            let transform =
                graphics::math::abs_transform(size.width as f64, size.height as f64)
                .trans(10.0, 100.0);
            clear([1.0; 4], &mut g);
            text::Text::new_color([0.0, 0.5, 0.0, 1.0], 32).draw(
                "Hello glium_graphics!",
                &mut glyph_cache,
                &default_draw_state(),
                transform,
                &mut g
            );
        }
        target.finish().unwrap();
    }
}

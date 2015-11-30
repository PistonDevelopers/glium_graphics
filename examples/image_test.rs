extern crate graphics;
extern crate glium;
extern crate glium_graphics;
extern crate image;
extern crate piston;
extern crate glutin_window;

use std::rc::Rc;
use std::cell::RefCell;
use std::path::Path;
use glium::{ Surface, Texture2d };
use glium::texture::RawImage2d;
use glium_graphics::{ Glium2d, GliumGraphics, DrawTexture, GliumWindow };
use piston::event_loop::*;
use piston::input::RenderEvent;
use piston::window::WindowSettings;
use glutin_window::{ GlutinWindow, OpenGL };

fn main() {
    let opengl = OpenGL::V3_2;
    let (w, h) = (300, 300);
    let ref window: Rc<RefCell<GlutinWindow>> = Rc::new(RefCell::new(
        WindowSettings::new("glium_graphics: image_test", [w, h])
        .exit_on_esc(true).build().unwrap()
    ));
    let ref glium_window = GliumWindow::new(window).unwrap();
    let rust_logo = DrawTexture::new({
        let image = image::open(&Path::new("assets/rust.png"))
            .unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        Texture2d::new(glium_window,
            RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions)).unwrap()
    });

    let mut g2d = Glium2d::new(opengl, glium_window);
    let transform = graphics::math::abs_transform(w as f64, h as f64);

    for _ in window.events().swap_buffers(false).filter_map(|e| e.render_args()) {
        use graphics::*;

        let mut target = glium_window.draw();

        {
            let mut g = GliumGraphics::new(&mut g2d, &mut target);

            clear(color::WHITE, &mut g);
            rectangle([1.0, 0.0, 0.0, 1.0],
                      [0.0, 0.0, 100.0, 100.0],
                      transform,
                      &mut g);
            rectangle([0.0, 1.0, 0.0, 0.3],
                      [50.0, 50.0, 100.0, 100.0],
                      transform,
                      &mut g);
            image(&rust_logo, transform.trans(100.0, 100.0), &mut g);
        }

        target.finish().unwrap();
    }
}

extern crate graphics;
extern crate glium;
extern crate glium_graphics;
extern crate image;
extern crate piston;
extern crate glutin_window;

use glium::Surface;
use glium_graphics::{
    Flip, Glium2d, GliumGraphics, GliumWindow, Texture, TextureSettings
};
use piston::input::RenderEvent;
use piston::window::WindowSettings;
use glutin_window::{ GlutinWindow, OpenGL };

fn main() {
    let opengl = OpenGL::V3_2;
    let (w, h) = (300, 300);
    let ref mut window: GliumWindow<GlutinWindow> =
        WindowSettings::new("glium_graphics: image_test", [w, h])
        .exit_on_esc(true).opengl(opengl).build().unwrap();

    let rust_logo = Texture::from_path(window, "assets/rust.png",
        Flip::None, &TextureSettings::new()).unwrap();

    let mut g2d = Glium2d::new(opengl, window);
    let transform = graphics::math::abs_transform(w as f64, h as f64);
    while let Some(e) = window.next() {
        use graphics::*;

        if let Some(_) = e.render_args() {
            let mut target = window.draw();
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
}

extern crate graphics;
extern crate glium;
extern crate glium_graphics;
extern crate image;
extern crate piston;

use glium_graphics::{
    Flip, Glium2d, GliumWindow, OpenGL, Texture, TextureSettings
};
use piston::input::RenderEvent;
use piston::event_loop::EventLoop;
use piston::window::WindowSettings;

fn main() {
    let opengl = OpenGL::V3_2;
    let (w, h) = (300, 300);
    let ref mut window: GliumWindow =
        WindowSettings::new("glium_graphics: image_test", [w, h])
        .exit_on_esc(true).graphics_api(opengl).build().unwrap();

    let rust_logo = Texture::from_path(window, "assets/rust.png",
        Flip::None, &TextureSettings::new()).unwrap();

    let mut g2d = Glium2d::new(opengl, window);
    window.set_lazy(true);
    while let Some(e) = window.next() {
        use graphics::*;

        if let Some(args) = e.render_args() {
            let mut target = window.draw();
            g2d.draw(&mut target, args.viewport(), |c, g| {
                clear(color::WHITE, g);
                rectangle([1.0, 0.0, 0.0, 1.0],
                          [0.0, 0.0, 100.0, 100.0],
                          c.transform, g);
                rectangle([0.0, 1.0, 0.0, 0.3],
                          [50.0, 50.0, 100.0, 100.0],
                          c.transform, g);
                image(&rust_logo, c.transform.trans(100.0, 100.0), g);
            });
            target.finish().unwrap();
        }

    }
}

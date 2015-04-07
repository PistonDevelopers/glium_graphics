extern crate graphics;
extern crate glium;
extern crate glutin;
extern crate glium_graphics;
extern crate image;

use std::path::Path;
use std::thread::sleep_ms;
use glium::{ DisplayBuild, Surface, Texture2d };
use glium_graphics::{ Glium2d, GliumGraphics, DrawTexture, OpenGL };

fn main() {
    let window = glutin::WindowBuilder::new()
        .with_dimensions(300, 300)
        .with_title("glium_graphics: image_test".to_string())
        .build_glium().unwrap();

    let rust_logo = DrawTexture::new({
        let image = image::open(&Path::new("assets/rust.png")).unwrap();
        Texture2d::new(&window, image)
    });

    let mut g2d = Glium2d::new(OpenGL::_3_2, &window);
    let (w, h) = window.get_framebuffer_dimensions();
    let transform = graphics::abs_transform(w as f64, h as f64);

    loop {
        let mut target = window.draw();
        {
            use graphics::*;

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
        target.finish();

        window.poll_events().last();
        if window.is_closed() {
            break
        }
        sleep_ms(15);
    }
}

#![feature(std_misc, old_io)]

extern crate graphics;
extern crate glutin;
extern crate glium_graphics;
extern crate image;

use std::path::Path;
use std::old_io::timer::sleep;
use std::time::duration::Duration;
use glium_graphics::glium::{ DisplayBuild, Surface };
use glium_graphics::{ Glium2d, GliumGraphics, OpenGL, TextureWithDevice };
use graphics::{ RelativeTransform, default_draw_state };

fn main() {
    let mut window = glutin::WindowBuilder::new()
        .with_dimensions(300, 300)
        .with_title("glium_graphics: image_test".to_string())
        .build_glium().unwrap();

    let rust_logo = TextureWithDevice::from_path(&mut window, &Path::new("assets/rust.png")).unwrap();

    let mut g2d = Glium2d::new(OpenGL::_3_2, &window);

    let draw_state = default_draw_state();

    loop {
        let mut target = window.draw();
        {
            let mut g = GliumGraphics::new(&mut g2d, &mut target);
            let (w, h) = window.get_framebuffer_dimensions();
            let transform = graphics::abs_transform(w as f64, h as f64);

            graphics::clear([1.0; 4], &mut g);
            graphics::Rectangle::new([1.0, 0.0, 0.0, 1.0])
                .draw([0.0, 0.0, 100.0, 100.0],
                      &draw_state,
                      transform,
                      &mut g);
            graphics::Rectangle::new([0.0, 1.0, 0.0, 0.3])
                .draw([50.0, 50.0, 100.0, 100.0],
                      &draw_state,
                      transform,
                      &mut g);
            graphics::image(&rust_logo, transform.trans(100.0, 100.0), &mut g);
        }
        target.finish();

        for _ in window.poll_events() {}
        if window.is_closed() {
            break
        }
        sleep(Duration::milliseconds(15));
    }
}

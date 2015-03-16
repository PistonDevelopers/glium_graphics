#![feature(std_misc, old_io, old_path)]

extern crate graphics;
extern crate glium;
extern crate glutin;
extern crate glium_graphics;
extern crate image;

use std::old_io::timer::sleep;
use std::time::duration::Duration;
use glium::{ DisplayBuild, Surface, Texture2d };
use glium_graphics::{ Glium2d, GliumGraphics, DrawTexture, OpenGL };
use graphics::{ RelativeTransform, default_draw_state };

fn main() {
    let window = glutin::WindowBuilder::new()
        .with_dimensions(300, 300)
        .with_title(format!("Image test"))
        .build_glium().unwrap();

    let rust_logo = DrawTexture::new({
        let image = image::open(&Path::new("assets/rust.png")).unwrap();
        Texture2d::new(&window, image)
    });

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

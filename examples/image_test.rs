#![feature(std_misc, io)]

extern crate graphics;
extern crate glium;
extern crate glutin;
extern crate glium_graphics;
extern crate image;


fn main() {
    use std::old_io::timer::sleep;
    use std::old_io::BufReader;
    use std::time::duration::Duration;
    use glium::{DisplayBuild, Surface, Texture2d};
    use glium_graphics::{GliumBackendSystem, GliumSurfaceBackEnd, DrawTexture};
    use graphics::{Context, RelativeTransform};

    let display = glutin::WindowBuilder::new()
        .with_dimensions(300, 300)
        .with_title(format!("Image test"))
        .build_glium().unwrap();

    let rust_logo = DrawTexture::new({
        let image =
            image::load(
                BufReader::new(include_bytes!("../assets/rust.png")),
                image::PNG
            ).unwrap();
        Texture2d::new(&display, image)
    });

    let mut system = GliumBackendSystem::new(&display);

    loop {
        let mut target = display.draw();
        {
            let mut backend = GliumSurfaceBackEnd::new(&mut system, &mut target);
            let context = Context::abs(300.0, 300.0);
            graphics::clear([1.0; 4], &mut backend);
            graphics::Rectangle::new([1.0, 0.0, 0.0, 1.0])
                .draw([0.0, 0.0, 100.0, 100.0], &context, &mut backend);
            graphics::Rectangle::new([0.0, 1.0, 0.0, 0.3])
                .draw(
                    [50.0, 50.0, 100.0, 100.0],
                    &context,
                    &mut backend
                );
            graphics::image(&rust_logo, &context.trans(100.0, 100.0), &mut backend);
        }
        target.finish();
        sleep(Duration::milliseconds(15));
    }
}

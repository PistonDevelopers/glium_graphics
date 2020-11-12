extern crate glium;
extern crate glium_graphics;
extern crate graphics;
extern crate image;
extern crate piston;

use glium_graphics::{Flip, Glium2d, GliumWindow, OpenGL, Texture, TextureSettings};
use piston::event_loop::EventLoop;
use piston::input::RenderEvent;
use piston::window::WindowSettings;

fn main() {
    let opengl = OpenGL::V3_2;
    let (w, h) = (300, 300);
    let ref mut window: GliumWindow = WindowSettings::new("glium_graphics: colored_image_test", [w, h])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let rust_logo = Texture::from_path(
        window,
        "assets/rust-white.png",
        Flip::None,
        &TextureSettings::new(),
    )
    .unwrap();

    let mut g2d = Glium2d::new(opengl, window);
    window.set_lazy(true);
    while let Some(e) = window.next() {
        use graphics::*;

        if let Some(args) = e.render_args() {
            let mut target = window.draw();
            g2d.draw(&mut target, args.viewport(), |c, g| {
                use graphics::triangulation::{tx, ty};

                let transform = c.transform.trans(0.0, 0.0);

                let tr = |p: [f64; 2]| [tx(transform, p[0], p[1]), ty(transform, p[0], p[1])];

                clear(color::WHITE, g);
                Rectangle::new([1.0, 0.0, 0.0, 1.0])
                    .draw([0.0, 0.0, 100.0, 100.0], &c.draw_state, c.transform, g);
                Rectangle::new([0.0, 1.0, 0.0, 0.3])
                    .draw([50.0, 50.0, 100.0, 100.0], &c.draw_state, c.transform, g);
                g.tri_list_uv_c(&c.draw_state, &rust_logo, |f| {
                    (f)(
                        &[
                            tr([0.0, 0.0]),
                            tr([300.0, 0.0]),
                            tr([0.0, 300.0]),

                            tr([300.0, 0.0]),
                            tr([0.0, 300.0]),
                            tr([300.0, 300.0]),
                        ],
                        &[
                            [0.0, 0.0],
                            [1.0, 0.0],
                            [0.0, 1.0],

                            [1.0, 0.0],
                            [0.0, 1.0],
                            [1.0, 1.0],
                        ],
                        &[
                            [1.0, 0.0, 0.0, 1.0],
                            [0.0, 1.0, 0.0, 1.0],
                            [0.0, 0.0, 1.0, 1.0],

                            [0.0, 00.0, 0.0, 1.0],
                            [0.0, 00.0, 0.0, 1.0],
                            [0.0, 00.0, 0.0, 1.0],
                        ]
                    )
                });
            });
            target.finish().unwrap();
        }
    }
}

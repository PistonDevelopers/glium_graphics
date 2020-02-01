extern crate glium;
extern crate glium_graphics;
extern crate graphics;
extern crate image as im;
extern crate piston;

use glium_graphics::{Flip, Glium2d, GliumWindow, OpenGL, Texture, TextureSettings, Wrap};
use piston::event_loop::EventLoop;
use piston::input::{Button, Key, PressEvent, RenderEvent};
use piston::window::WindowSettings;

fn main() {
    println!("Press U to change the texture wrap mode for the u coordinate");
    println!("Press V to change the texture wrap mode for the v coordinate");

    let opengl = OpenGL::V3_2;
    let (w, h) = (600, 600);
    let mut window: GliumWindow = WindowSettings::new("glium_graphics: texture_wrap", [w, h])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();
    window.set_lazy(true);

    // Set up wrap modes
    let wrap_modes = [
        Wrap::ClampToEdge,
        Wrap::ClampToBorder,
        Wrap::Repeat,
        Wrap::MirroredRepeat,
    ];
    let mut ix_u = 0;
    let mut ix_v = 0;
    let mut texture_settings = TextureSettings::new();
    texture_settings.set_border_color([0.0, 0.0, 0.0, 1.0]);

    let mut rust_logo = Texture::from_path(
        &mut window,
        "assets/rust.png",
        Flip::None,
        &texture_settings,
    )
    .unwrap();

    let mut g2d = Glium2d::new(opengl, &mut window);
    while let Some(e) = window.next() {
        if let Some(args) = e.render_args() {
            use graphics::*;
            let mut target = window.draw();
            g2d.draw(&mut target, args.viewport(), |_c, g| {
                clear([1.0; 4], g);
                let points = [[0.5, 0.5], [-0.5, 0.5], [-0.5, -0.5], [0.5, -0.5]];
                // (0, 1, 2) and (0, 2, 3)
                let uvs = [
                    [4.0, 0.0],
                    [0.0, 0.0],
                    [0.0, 4.0],
                    [4.0, 0.0],
                    [0.0, 4.0],
                    [4.0, 4.0],
                ];
                let mut verts = [[0.0, 0.0]; 6];
                let indices_points: [usize; 6] = [0, 1, 2, 0, 2, 3];
                for (ixv, &ixp) in (0..6).zip(indices_points.into_iter()) {
                    verts[ixv] = points[ixp];
                }
                g.tri_list_uv(&DrawState::new_alpha(), &[1.0; 4], &rust_logo, |f| {
                    f(&verts, &uvs)
                });
            });
            target.finish().unwrap();
        }

        if let Some(Button::Keyboard(Key::U)) = e.press_args() {
            ix_u = (ix_u + 1) % wrap_modes.len();
            texture_settings.set_wrap_u(wrap_modes[ix_u]);
            rust_logo = Texture::from_path(
                &mut window,
                "assets/rust.png",
                Flip::None,
                &texture_settings,
            )
            .unwrap();
            println!(
                "Changed texture wrap mode for u coordinate to: {:?}",
                wrap_modes[ix_u]
            );
        }

        if let Some(Button::Keyboard(Key::V)) = e.press_args() {
            ix_v = (ix_v + 1) % wrap_modes.len();
            texture_settings.set_wrap_v(wrap_modes[ix_v]);
            rust_logo = Texture::from_path(
                &mut window,
                "assets/rust.png",
                Flip::None,
                &texture_settings,
            )
            .unwrap();
            println!(
                "Changed texture wrap mode for v coordinate to: {:?}",
                wrap_modes[ix_v]
            );
        }
    }
}

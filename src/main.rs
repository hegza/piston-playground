#[macro_use]
extern crate ecs;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

mod component;
mod app;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ OpenGL };
use app::App;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    const WIN_WIDTH: u32 = 200;
    const WIN_HEIGHT: u32 = 200;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "spinning-square",
            [WIN_WIDTH, WIN_HEIGHT]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App::init(WIN_WIDTH, WIN_HEIGHT, opengl);

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}

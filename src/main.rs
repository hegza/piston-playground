#[macro_use]
extern crate specs;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

mod app;
mod component;
mod system;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use piston::window::AdvancedWindow;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ OpenGL };
use app::App;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    const WIN_WIDTH: u32 = 200;
    const WIN_HEIGHT: u32 = 200;

    let mut app = App::init(WIN_WIDTH, WIN_HEIGHT, opengl);
    app.run();
}

use piston::input::*;
use opengl_graphics::{GlGraphics, OpenGL};
use component::transform::Transform;
use system::rotate::Rotate;
use specs::{Entity, Planner, World, Join};
use glutin_window::GlutinWindow;
use piston::window::{WindowSettings, AdvancedWindow};
use piston::event_loop::*;

pub struct App {
    pub gl: GlGraphics, // OpenGL for drawing backend
    pub entity: Entity,
    pub planner: Planner<f64>,
    pub window: GlutinWindow,
}

impl App {
    pub fn init(width: u32, height: u32, opengl: OpenGL) -> App {
        // Create an Glutin window.
        let mut window: GlutinWindow = WindowSettings::new(
            "spinning-square",
            [width, height]
            )
            .opengl(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

        // Create a new game and run it.
        let mut w = World::new();

        // Register all components
        w.register::<Transform>();

        // Create entity
        // TODO: use nalgebra matrix (check nphysics, ncollide)
        let e = w.create_now().with(
            Transform {
                x: (width/2) as f64, y: (height/2) as f64, scale_x: 1.0, scale_y: 0.5, rotation: 0.0, pivot_x: 25.0, pivot_y: 25.0
                }
            ).build();

        let rotate_system = Rotate {};
        let mut planner = Planner::new(w, 4);
        planner.add_system(rotate_system, "rotate", 0);

        App {
            gl: GlGraphics::new(opengl),
            entity: e,
            planner: planner,
            window: window,
        }
    }

    pub fn run(&mut self) {
        let mut capture_cursor = false;
        let mut events = self.window.events();
        while let Some(e) = events.next(&mut self.window) {
            if let Some(r) = e.render_args() {
                self.render(&r);
            }

            if let Some(u) = e.update_args() {
                self.update(&u);
            }

            // Handle keyboard
            if let Some(Button::Keyboard(key)) = e.press_args() {
                if key == Key::C {
                    println!("Turned capture cursor on");
                    capture_cursor = !capture_cursor;
                    self.window.set_capture_cursor(capture_cursor);
                }
                if key == Key::W {
                    //self.entity.
                }

                println!("Pressed keyboard key '{:?}'", key);
            };

        }
    }

    pub fn render(&mut self, args: &RenderArgs) {

        // wait waits for all scheduled systems to finish
        // If wait is not called, all systems are run in parallel, waiting on locks
        self.planner.wait();

        let transforms = self.planner.mut_world().read::<Transform>();

        for tfm in transforms.iter() {
            use graphics::*;

            const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
            const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

            let square = rectangle::square(0.0, 0.0, 50.0);

            self.gl.draw(args.viewport(), |c, gl| {
                // Clear the screen.
                clear(GREEN, gl);

                let transform = c.transform
                    .trans(tfm.x, tfm.y)
                    .rot_rad(tfm.rotation)
                    .scale(tfm.scale_x, tfm.scale_y)
                    .trans(-tfm.pivot_x, -tfm.pivot_y);

                // Draw a box rotating around a point.
                rectangle(RED, square, transform, gl);
            });
        }


    }

    pub fn update(&mut self, args: &UpdateArgs) {
        // TODO: gather input

        // Update systems
        self.planner.dispatch(args.dt);
    }
}

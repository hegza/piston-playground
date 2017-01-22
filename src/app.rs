use piston::input::*;
use opengl_graphics::{ GlGraphics, OpenGL };
use ecs::{World,Entity,BuildData};
use component::transform::Transform;

components! {
    struct MyComponents {
        #[hot] transform: Transform
    }
}

systems! {
    struct MySystems<MyComponents, ()>;
}

pub struct App {
    pub gl: GlGraphics, // OpenGL for drawing backend
    pub world: World<MySystems>,
    pub entity: Entity
}

impl App {

    pub fn init(width: u32, height: u32, opengl: OpenGL) -> App {
        let mut world = World::<MySystems>::new();
        App {
            gl: GlGraphics::new(opengl),
            entity: world.create_entity(
                    |entity: BuildData<MyComponents>, data: &mut MyComponents| {
                        data.transform.add(&entity,
                            Transform {
                                 x: (width/2) as f64, y: (height/2) as f64, scale_x: 1.0, scale_y: 0.5, rotation: 0.0, pivot_x: 25.0, pivot_y: 25.0
                             });
                    }),
            world: world
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let mut rotation = 0.0;
        // Position
        let mut x = 0.0;
        let mut y = 0.0;
        // Pivot
        let mut px = 0.0;
        let mut py = 0.0;
        // Scale
        let mut sx = 0.0;
        let mut sy = 0.0;
        self.world.with_entity_data(&self.entity, |entity, data| {
            let tfm = data.transform[entity];
            rotation = tfm.rotation;
            x = tfm.x;
            y = tfm.y;
            px = tfm.pivot_x;
            py = tfm.pivot_y;
            sx = tfm.scale_x;
            sy = tfm.scale_y;
        });

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            // TODO: move to system
            let transform = c.transform
                .trans(x, y)
                .rot_rad(rotation)
                .scale(sx, sy)
                .trans(-px, -py);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.world.with_entity_data(&self.entity, |entity, data| {
            // Rotate 2 radians per second.
            data.transform[entity].rotation += 2.0 * args.dt;
        });
    }

}

use specs::*;
use component::transform::Transform;

pub struct Rotate;

impl System<f64> for Rotate {
    fn run(&mut self, arg: RunArg, dt: f64) {
        let mut tfms = arg.fetch(|w| {
            (w.write::<Transform>())
        });

        for tfm in (&mut tfms).iter() {
            // Rotate 2 radians per second.
            tfm.rotation += 2.0 * dt;
        }
    }
}

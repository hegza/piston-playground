use specs::*;

#[derive(Debug, PartialEq)]
pub struct Transform {
    pub x: f64,
    pub y: f64,
    pub scale_x: f64,
    pub scale_y: f64,
    pub rotation: f64,
    pub pivot_x: f64,
    pub pivot_y: f64
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            x: 0.,
            y: 0.,
            scale_x: 0.,
            scale_y: 0.,
            rotation: 0.,
            pivot_x: 0.,
            pivot_y: 0.
        }
    }
}

impl Component for Transform {
    type Storage = VecStorage<Transform>;
}

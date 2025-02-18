use glam::{Vec3, Vec4};
use rand::Rng;

#[derive(Clone, Copy, Debug, Default)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl From<Color> for Vec3 {
    fn from(val: Color) -> Self {
        Self {
            x: val.r,
            y: val.g,
            z: val.b,
        }
    }
}
impl From<Color> for Vec4 {
    fn from(val: Color) -> Self {
        Self::new(val.r, val.g, val.b, val.a)
    }
}

impl Color {
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
    pub fn random() -> Self {
        let mut rng = rand::rng();
        Self {
            r: rng.random_range(0.0..=1.0),
            g: rng.random_range(0.0..=1.0),
            b: rng.random_range(0.0..=1.0),
            a: 1.0,
        }
    }
}

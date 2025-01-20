use glam::{Vec3, Vec4};

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
}

#[derive(Clone, Copy, Debug, Default)]
struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Color {
    fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

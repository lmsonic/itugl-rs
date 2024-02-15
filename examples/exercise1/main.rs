use itugl::application::{application::Application, window::Window};

pub struct TerrainApplication {
    window: Window,
    delta_time: f32,
    current_time: f32,
}

impl Application for TerrainApplication {
    fn new(width: u32, height: u32, title: &str) -> Self {
        Self {
            window: Window::new(width, height, title, glfw::WindowMode::Windowed),
            delta_time: 0.0,
            current_time: 0.0,
        }
    }
    fn window(&self) -> &itugl::application::window::Window {
        &self.window
    }

    fn window_mut(&mut self) -> &mut itugl::application::window::Window {
        &mut self.window
    }

    fn delta_time(&self) -> f32 {
        self.delta_time
    }

    fn delta_time_mut(&mut self) -> &mut f32 {
        &mut self.delta_time
    }

    fn current_time(&self) -> f32 {
        self.current_time
    }

    fn current_time_mut(&mut self) -> &mut f32 {
        &mut self.current_time
    }

    fn initialize(&mut self) {}

    fn update(&mut self) {}

    fn render(&mut self) {}
}
fn main() {
    let app = TerrainApplication::new(1024, 1024, "TerrainDemo");
    app.run();
}

use std::ffi::CString;

use itugl::{
    application::{application::Application, window::Window},
    shader::{Program, Shader},
};

#[derive(Debug)]
pub struct SkeletonApplication {
    window: Window,
    delta_time: f32,
    current_time: f32,
}

impl Application for SkeletonApplication {
    fn new(width: u32, height: u32, title: &str) -> Self {
        Self {
            window: Window::new(width, height, title, glfw::WindowMode::Windowed),
            delta_time: 0.0,
            current_time: 0.0,
        }
    }
    fn window(&self) -> &Window {
        &self.window
    }

    fn window_mut(&mut self) -> &mut Window {
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
    let app = SkeletonApplication::new(1024, 1024, "SkeletonDemo");
    app.run();
}
#[allow(dead_code)]
fn build_shaders() -> Program {
    let cstr = CString::new(include_str!("vert.vert")).unwrap();
    let vertex_shader = Shader::from_vert_source(&cstr).unwrap();

    let cstr = CString::new(include_str!("frag.frag")).unwrap();

    let fragment_shader = Shader::from_frag_source(&cstr).unwrap();

    Program::from_shaders(&[vertex_shader, fragment_shader]).unwrap()
}

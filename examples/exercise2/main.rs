use std::ffi::CString;

use glam::Vec2;
use itugl::{
    application::{application::Application, window::Window},
    core::{color::Color, data::Type},
    geometry::vertex_attribute::VertexAttribute,
    shader::{Location, Program, Shader},
};
#[derive(Clone, Copy, Default)]
#[repr(C)]
struct Particle {
    position: Vec2,
    size: f32,
    birth: f32,
    duration: f32,
    color: Color,
    velocity: Vec2,
}

const vertex_attributes: [VertexAttribute; 6] = [
    VertexAttribute::new(Type::Float, 2, false), // position
    VertexAttribute::new(Type::Float, 1, false), // size
    VertexAttribute::new(Type::Float, 1, false), // birth
    VertexAttribute::new(Type::Float, 1, false), // duration
    VertexAttribute::new(Type::Float, 4, false), // color
    VertexAttribute::new(Type::Float, 2, false), // velocity
];

#[derive(Debug)]
pub struct ParticlesApplication {
    window: Window,
    program: Program,
    delta_time: f32,
    current_time: f32,
    current_time_uniform: Location,
    current_gravity_uniform: Location,
    mouse_position: Vec2,
    particle_count: usize,
    particle_capacity: usize,
}
impl ParticlesApplication {
    fn initialize_geometry(&self) {}
}

impl Application for ParticlesApplication {
    fn new(width: u32, height: u32, title: &str) -> Self {
        let mut window = Window::new(width, height, title, glfw::WindowMode::Windowed);
        window.enable_feature(gl::PROGRAM_POINT_SIZE);

        let program = build_shaders();

        Self {
            delta_time: 0.0,
            current_time: 0.0,
            current_time_uniform: program.get_uniform_location(c"CurrentTime"),
            current_gravity_uniform: program.get_uniform_location(c"Gravity"),
            mouse_position: window.get_mouse_position(false),
            particle_count: 0,
            particle_capacity: 0,
            window,
            program,
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

    fn initialize(&mut self) {
        self.initialize_geometry();
        self.window.enable_feature(gl::BLEND);
        unsafe { gl::BlendFunc(gl::SRC_ALPHA, gl::ONE) };
        self.window.set_vsync(true);
    }

    fn update(&mut self) {}

    fn render(&mut self) {}
}

fn main() {
    let app = ParticlesApplication::new(1024, 1024, "ParticlesDemo");
    app.run();
}

fn build_shaders() -> Program {
    let vertex_shader =
        Shader::from_vert_source(&CString::new(include_str!("vert.vert")).unwrap()).unwrap();

    let fragment_shader =
        Shader::from_frag_source(&CString::new(include_str!("frag.frag")).unwrap()).unwrap();

    Program::from_shaders(&[vertex_shader, fragment_shader]).unwrap()
}

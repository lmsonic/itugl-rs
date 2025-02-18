use std::{ffi::CString, mem};

use gl::types::GLsizei;
use glam::Vec2;
use glfw::Action;
use itugl::{
    application::{application::Application, window::Window},
    core::{
        buffer_object::{BufferObject, Usage},
        color::Color,
        data::Type,
        object::Object,
    },
    error::check_gl_error,
    geometry::{
        vertex_array_object::VertexArrayObject, vertex_attribute::VertexAttribute,
        vertex_buffer_object::VertexBufferObject,
    },
    shader::{Location, Program, Shader},
};
use rand::Rng;
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

const VERTEX_ATTRIBUTES: [VertexAttribute; 6] = [
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
    vao: VertexArrayObject,
    vbo: VertexBufferObject,
}
impl ParticlesApplication {
    fn emit_particle(
        &mut self,
        position: Vec2,
        size: f32,
        duration: f32,
        color: Color,
        velocity: Vec2,
    ) {
        let particle = Particle {
            position,
            size,
            birth: self.current_time,
            duration,
            color,
            velocity,
        };
        let particle_index = self.particle_count % self.particle_capacity;
        let offset = particle_index * mem::size_of::<Particle>();
        self.vbo.update_data(&[particle], offset as isize);
        self.particle_count += 1;
    }
}

impl Application for ParticlesApplication {
    fn new(width: u32, height: u32, title: &str) -> Self {
        let mut window = Window::new(width, height, title, glfw::WindowMode::Windowed);
        window.enable_feature(gl::PROGRAM_POINT_SIZE);
        window.enable_feature(gl::BLEND);
        unsafe { gl::BlendFunc(gl::SRC_ALPHA, gl::ONE) };
        window.set_vsync(true);
        let program = build_shaders();
        let particle_capacity = 2048;
        // initialize geometry
        let vbo = VertexBufferObject::new();
        vbo.reserve_data::<Particle>(particle_capacity, Usage::DynamicDraw);

        let vao = VertexArrayObject::new();
        vao.bind();
        let stride = mem::size_of::<Particle>() as GLsizei;
        let mut offset = 0;
        for (location, attribute) in VERTEX_ATTRIBUTES.into_iter().enumerate() {
            vao.set_attribute(location as u32, &attribute, offset, stride);
            offset += attribute.get_size();
        }
        vao.unbind();
        Self {
            delta_time: 0.0,
            current_time: 0.0,
            current_time_uniform: program.get_uniform_location(c"CurrentTime"),
            current_gravity_uniform: program.get_uniform_location(c"Gravity"),
            mouse_position: window.get_mouse_position(false),
            particle_count: 0,
            particle_capacity,
            window,
            program,
            vao,
            vbo,
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

    fn update(&mut self) {
        let mouse_pos = self.window.get_mouse_position(true);

        if self.window.is_mouse_button_pressed(glfw::MouseButtonLeft) == Action::Press {
            let mut rng = rand::rng();
            let size = rng.random_range(10.0..=30.0);
            let duration = rng.random_range(1.0..=2.0);
            let color = Color::random();
            let velocity = 0.5 * (mouse_pos - self.mouse_position) * self.delta_time;
            self.emit_particle(mouse_pos, size, duration, color, velocity);
        }
        self.mouse_position = mouse_pos;
    }

    fn render(&mut self) {
        self.window.clear_color(0.0, 0.0, 0.0, 0.0);
        self.program.set_used();
        self.program
            .set_uniform1f(self.current_time_uniform, self.current_time);
        self.program
            .set_uniform1f(self.current_gravity_uniform, -9.8);
        let particle_count = self.particle_count as i32;
        let particle_capacity = self.particle_capacity as i32;
        self.vao.bind();
        unsafe { gl::DrawArrays(gl::POINTS, 0, particle_count.min(particle_capacity)) };
    }
}

fn main() {
    let app = ParticlesApplication::new(1024, 1024, "ParticlesDemo");
    app.run();
}

fn build_shaders() -> Program {
    let cstr = CString::new(include_str!("vert.vert")).unwrap();
    let vertex_shader = Shader::from_vert_source(&cstr).unwrap();

    let cstr = CString::new(include_str!("frag.frag")).unwrap();
    let fragment_shader = Shader::from_frag_source(&cstr).unwrap();

    Program::from_shaders(&[vertex_shader, fragment_shader]).unwrap()
}

use std::{f32::consts::PI, ffi::CString, ptr::null};

use gl::types::GLsizei;
use glfw::{Action, Context, Key};
use itugl::{
    application,
    core::{
        buffer_object::{BufferObject, Usage},
        object::Object,
    },
    error::check_gl_error,
    geometry::{
        element_buffer_object::ElementBufferObject, vertex_array_object::VertexArrayObject,
        vertex_attribute::VertexAttribute, vertex_buffer_object::VertexBufferObject,
    },
    shader::{self, Program, Shader},
};

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

fn main() {
    // Create a windowed mode window and its OpenGL context
    let mut window = application::window::Window::new(
        SCR_WIDTH,
        SCR_HEIGHT,
        "LearnOpenGL",
        glfw::WindowMode::Windowed,
    );

    // set up vertex data (and buffer(s)) and configure vertex attributes
    // ------------------------------------------------------------------
    let sides = 16;
    let length = 0.5 * f32::sqrt(2.0);

    // Using std::array instead of regular arrays makes sure we don't access out of range
    let mut vertices = vec![0.0; 3 * (sides + 1)];
    let mut indices: Vec<u32> = vec![0; 3 * sides];

    // Loop over 2*PI with N sides
    let delta_angle = 2.0 * PI / sides as f32;
    for i in 0..sides {
        let angle = i as f32 * delta_angle;
        vertices[3 * i + 3] = f32::sin(angle) * length;
        vertices[3 * i + 4] = f32::cos(angle) * length;
        vertices[3 * i + 5] = 0.0;

        indices[3 * i] = 0;
        indices[3 * i + 1] = (i + 1) as u32;
        indices[3 * i + 2] = (i + 2) as u32;
    }

    // Connect last index with vertex 1 to close the circle
    indices[3 * sides - 1] = 1;

    let shader_program = build_shader_program();

    let vbo = VertexBufferObject::new();

    vbo.allocate_data(&vertices, Usage::StaticDraw);

    let ebo = ElementBufferObject::new();
    ebo.allocate_data(&indices, Usage::StaticDraw);

    let vao = VertexArrayObject::new();
    let attributes = VertexAttribute::new(itugl::core::data::Type::Float, 3, false);
    vao.bind();
    vbo.bind();
    ebo.bind();
    vao.set_attribute(0, &attributes, 0, 0);
    vbo.unbind();
    vao.unbind();
    ebo.unbind();

    let size = window.inner_window.get_size();
    window.set_viewport(size.0, size.1);
    window.clear(0.3, 0.3, 0.5, 1.0);

    // Loop until the user closes the window
    while !window.inner_window.should_close() {
        window.clear(0.3, 0.3, 0.5, 1.0);

        // draw our first triangle
        shader_program.set_used();
        vao.bind();
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                indices.len() as GLsizei,
                gl::UNSIGNED_INT,
                null(),
            )
        };
        check_gl_error();

        // Swap front and back buffers
        window.inner_window.swap_buffers();
        window.glfw_mut().poll_events();
        for (_, event) in glfw::flush_messages(&window.events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.inner_window.set_should_close(true)
                }
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    // make sure the viewport matches the new window dimensions; note that width and
                    // height will be significantly larger than specified on retina displays.
                    window.set_viewport(width, height);
                }

                _ => {}
            }
        }
    }
}

fn build_shader_program() -> Program {
    let vertex_shader =
        Shader::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap()).unwrap();

    let fragment_shader =
        Shader::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap()).unwrap();

    shader::Program::from_shaders(&[vertex_shader, fragment_shader]).unwrap()
}

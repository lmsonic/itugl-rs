use std::{
    ffi::{CStr, CString},
    mem,
    ptr::null,
};

use gl::EnableVertexAttribArray;
use glfw::{fail_on_errors, Action, Context, Key};
use itugl::{
    application,
    core::object::Object,
    geometry::{vertex_array_object::VertexArrayObject, vertex_attribute::VertexAttribute},
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

    let vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

    let shader_program = build_shader_program();

    let mut vbo: gl::types::GLuint = 0;
    unsafe { gl::GenBuffers(1, &mut vbo) };

    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW,
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    };

    let vao = VertexArrayObject::new();
    let attributes = VertexAttribute::new(itugl::core::data::Type::Float, 3, false);
    vao.bind();
    unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, vbo) };
    vao.set_attribute(0, &attributes, 0, 0);
    unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, 0) };
    vao.unbind();

    let size = window.inner_window.get_size();
    window.set_viewport(size.0, size.1);
    window.clear(0.3, 0.3, 0.5, 1.0);

    // Loop until the user closes the window
    while !window.inner_window.should_close() {
        window.clear(0.3, 0.3, 0.5, 1.0);

        // draw our first triangle
        shader_program.set_used();
        vao.bind();
        unsafe { gl::DrawArrays(gl::TRIANGLES, 0, 3) };

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

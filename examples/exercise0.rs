use std::{
    ffi::{CStr, CString},
    mem,
    ptr::null,
};

use gl::EnableVertexAttribArray;
use glfw::{fail_on_errors, Action, Context, Key};
use itugl::{
    application,
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

    let mut vao: gl::types::GLuint = 0;
    unsafe { gl::GenVertexArrays(1, &mut vao) };
    unsafe {
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,         // index of the generic vertex attribute ("layout (location = 0)")
            3,         // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (3 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null(),                                     // offset of the first component
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    };
    let size = window.inner_window.get_size();
    unsafe {
        gl::Viewport(0, 0, size.0, size.1);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    // Loop until the user closes the window
    while !window.inner_window.should_close() {
        for (_, event) in glfw::flush_messages(&window.events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.inner_window.set_should_close(true)
                }
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    // make sure the viewport matches the new window dimensions; note that width and
                    // height will be significantly larger than specified on retina displays.
                    unsafe { gl::Viewport(0, 0, width, height) }
                }

                _ => {}
            }
        }
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };

        // draw our first triangle
        shader_program.set_used();
        unsafe { gl::BindVertexArray(vao) };
        unsafe { gl::DrawArrays(gl::TRIANGLES, 0, 3) };

        // Swap front and back buffers
        window.inner_window.swap_buffers();
        window.glfw_mut().poll_events();
    }
}

fn build_shader_program() -> Program {
    let vertex_shader =
        Shader::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap()).unwrap();

    let fragment_shader =
        Shader::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap()).unwrap();

    shader::Program::from_shaders(&[vertex_shader, fragment_shader]).unwrap()
}

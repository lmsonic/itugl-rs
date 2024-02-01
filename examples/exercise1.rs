use std::{
    ffi::{CStr, CString},
    ptr::null,
};

use glfw::{fail_on_errors, Action, Context, Key};

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

fn main() {
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw
        .create_window(
            SCR_WIDTH,
            SCR_HEIGHT,
            "LearnOpenGL",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();

    let vertices = [
        -0.5, -0.5, 0.0, // let
        0.5, -0.5, 0.0, // right
        0.0, 0.5, 0.0, // top
    ];

    // Loop until the user closes the window
    while !window.should_close() {
        // Swap front and back buffers
        window.swap_buffers();

        // Poll for and process events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                _ => {}
            }
        }
    }
}

fn build_shader_program() -> u32 {
    let vertex_shader_source = &CStr::from_bytes_with_nul(
        b"#version 330 core
layout (location = 0) in vec3 aPos;
void main()
{
gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
};",
    )
    .unwrap();
    let fragment_shader_source = &CStr::from_bytes_with_nul(
        b"#version 330 core
out vec4 FragColor;
void main()
{
FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
};",
    )
    .unwrap();

    // vertex shader
    let vertex_shader = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
    unsafe { gl::ShaderSource(vertex_shader, 1, &vertex_shader_source.as_ptr(), null()) };
    unsafe {
        gl::CompileShader(vertex_shader);
    };

    // check for shader compile errors
    let mut success: gl::types::GLint = 1;
    let mut buffer: Vec<u8> = Vec::with_capacity(512);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(512));
    // convert buffer to CString
    let error: CString = unsafe { CString::from_vec_unchecked(buffer) };
    unsafe { gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success) };
    if success == 0 {
        unsafe {
            gl::GetShaderInfoLog(
                vertex_shader,
                512,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            )
        };
        eprintln!("ERROR::SHADER::VERTEX::COMPILATION_FAILED {error:?}");
    }

    // fragment shader
    let fragment_shader = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
    unsafe { gl::ShaderSource(vertex_shader, 1, &vertex_shader_source.as_ptr(), null()) };
    unsafe {
        gl::CompileShader(vertex_shader);
    };

    // check for shader compile errors
    let mut success: gl::types::GLint = 1;
    let mut buffer: Vec<u8> = Vec::with_capacity(512);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(512));
    // convert buffer to CString
    unsafe { gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success) };
    if success == 0 {
        unsafe {
            gl::GetShaderInfoLog(
                vertex_shader,
                512,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            )
        };
        eprintln!("ERROR::SHADER::FRAGMENT::COMPILATION_FAILED {error:?}");
    }

    // link shaders
    let shader_program = unsafe { gl::CreateProgram() };
    unsafe { gl::AttachShader(shader_program, vertex_shader) };
    unsafe { gl::AttachShader(shader_program, fragment_shader) };
    unsafe { gl::LinkProgram(shader_program) };
    // check for linking errors
    unsafe { gl::GetShaderiv(vertex_shader, gl::LINK_STATUS, &mut success) };
    if success == 0 {
        unsafe {
            gl::GetShaderInfoLog(
                vertex_shader,
                512,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            )
        };
        eprintln!("ERROR::SHADER::PROGRAM::LINKING_FAILED {error:?}");
    }
    unsafe { gl::DeleteShader(vertex_shader) };
    unsafe { gl::DeleteShader(fragment_shader) };
    return shader_program;
}

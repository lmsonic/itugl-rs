use std::{
    ffi::{CStr, CString},
    mem,
    ptr::null,
};

use gl::EnableVertexAttribArray;
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
    gl::load_with(|s| glfw.get_proc_address_raw(s));
    // Make the window's context current
    window.make_current();

    let vertices = [
        -0.5, -0.5, 0.0, // let
        0.5, -0.5, 0.0, // right
        0.0, 0.5, 0.0, // top
    ];

    let shader_program = build_shader_program();

    let mut vao = 0;
    let mut vbo = 0;
    unsafe { gl::GenVertexArrays(1, &mut vao) };
    unsafe { gl::GenBuffers(1, &mut vbo) };
    // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
    unsafe { gl::BindVertexArray(vao) };

    unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, vbo) };
    unsafe {
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW,
        )
    };

    unsafe {
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * mem::size_of::<f32>() as i32,
            std::ptr::null(),
        )
    };
    unsafe { gl::EnableVertexAttribArray(0) };

    // note that this is allowed, the call to glVertexAttribPointer registered VBO as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
    unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, 0) };

    // You can unbind the VAO afterwards so other VAO calls won't accidentally modify this VAO, but this rarely happens. Modifying other
    // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
    unsafe { gl::BindVertexArray(0) };

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
};\0",
    )
    .unwrap();
    let fragment_shader_source = &CStr::from_bytes_with_nul(
        b"#version 330 core
out vec4 FragColor;
void main()
{
FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
};\0",
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

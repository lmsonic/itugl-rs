use std::{ffi::CString, mem, ptr::null};

use glfw::{Action, Key};
use itugl::{
    application::{application::Application, window::Window},
    core::{
        buffer_object::{BufferObject, Usage},
        data,
        object::Object,
    },
    error::check_gl_error,
    geometry::{
        element_buffer_object::ElementBufferObject, vertex_array_object::VertexArrayObject,
        vertex_attribute::VertexAttribute, vertex_buffer_object::VertexBufferObject,
    },
    shader::{Program, Shader},
};
use noise::{Fbm, MultiFractal, NoiseFn, Perlin};

#[derive(Clone, Copy, Default)]
#[repr(C)]
struct Vector2 {
    x: f32,
    y: f32,
}

impl Vector2 {
    const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
#[derive(Clone, Copy, Default)]
#[repr(C)]
struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector3 {
    const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    fn normalize(&self) -> Self {
        let length = f32::sqrt(
            self.z
                .mul_add(self.z, self.x.mul_add(self.x, self.y * self.y)),
        );
        Self {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }
}

#[derive(Clone, Copy, Default)]
#[repr(C)]
struct Vertex {
    position: Vector3,
    tex_coord: Vector2,
    color: Vector3,
    normal: Vector3,
}

#[derive(Debug)]
pub struct TerrainApplication {
    window: Window,
    delta_time: f32,
    current_time: f32,
    program: Program,
    grid_x: u32,
    grid_y: u32,
    vbo: VertexBufferObject,
    vao: VertexArrayObject,
    ebo: ElementBufferObject,
}

impl Application for TerrainApplication {
    fn new(width: u32, height: u32, title: &str) -> Self {
        Self {
            window: Window::new(width, height, title, glfw::WindowMode::Windowed),
            delta_time: 0.0,
            current_time: 0.0,
            program: build_shaders(),
            grid_x: 256,
            grid_y: 256,
            vbo: VertexBufferObject::new(),
            vao: VertexArrayObject::new(),
            ebo: ElementBufferObject::new(),
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
        // Create containers for the vertex data
        let mut vertices: Vec<Vertex> = vec![];

        // Create container for the element data
        let mut indices: Vec<u32> = vec![];

        // Grid scale to convert the entire grid to size 1x1
        let scale = Vector2::new(1.0 / self.grid_x as f32, 1.0 / self.grid_y as f32);
        // Number of columns and rows
        let column_count = self.grid_x + 1;
        let row_count = self.grid_y + 1;
        let fbm: Fbm<Perlin> = Fbm::new(1)
            .set_lacunarity(1.9)
            .set_octaves(8)
            .set_persistence(0.5)
            .set_frequency(0.5);
        for j in 0..column_count {
            for i in 0..row_count {
                // Vertex data for this vertex only

                let mut vertex = Vertex::default();
                let x = (i as f32).mul_add(scale.x, -0.5);
                let y = (j as f32).mul_add(scale.y, -0.5);
                let z = fbm.get([f64::from(x) * 2.0, f64::from(y) * 2.0, 0.0]) as f32; //, 1.9, 0.5, 8) * 0.5;
                vertex.position = Vector3::new(x, y, z);
                vertex.tex_coord = Vector2::new(i as f32, j as f32);
                vertex.color = get_color_from_height(z);
                vertex.normal = Vector3::new(0.0, 0.0, 1.0); // Actual value computed after all vertices are created
                vertices.push(vertex);

                // Index data for quad formed by previous vertices and current
                if i > 0 && j > 0 {
                    let top_right = j * column_count + i; // Current vertex
                    let top_left = top_right - 1;
                    let bottom_right = top_right - column_count;
                    let bottom_left = bottom_right - 1;

                    //Triangle 1
                    indices.push(bottom_left);
                    indices.push(bottom_right);
                    indices.push(top_left);

                    //Triangle 2
                    indices.push(bottom_right);
                    indices.push(top_left);
                    indices.push(top_right);
                }
            }
        }
        for j in 0..column_count {
            for i in 0..row_count {
                // Get the vertex at (i, j)
                let index = (j * column_count + i) as usize;
                let mut vertex = vertices[index];

                // Compute the delta in X
                let prev_x = if i > 0 { index - 1 } else { index };
                let next_x = if i < self.grid_x { index + 1 } else { index };
                let delta_height_x = vertices[next_x].position.z - vertices[prev_x].position.z;
                let delta_x = vertices[next_x].position.x - vertices[prev_x].position.x;
                let x = delta_height_x / delta_x;

                // Compute the delta in Y
                let prev_y = if j > 0 {
                    index - column_count as usize
                } else {
                    index
                };
                let next_y = if j < self.grid_y {
                    index + column_count as usize
                } else {
                    index
                };
                let delta_height_y = vertices[next_y].position.z - vertices[prev_y].position.z;
                let delta_y = vertices[next_y].position.y - vertices[prev_y].position.y;
                let y = delta_height_y / delta_y;

                // Compute the normal
                vertex.normal = Vector3::new(x, y, 1.0).normalize();
            }
        }
        // Declare attributes
        let position_attribute = VertexAttribute::new(data::Type::Float, 3, false);
        let tex_coord_attribute = VertexAttribute::new(data::Type::Float, 2, false);
        let color_attribute = VertexAttribute::new(data::Type::Float, 3, false);
        let normal_attribute = VertexAttribute::new(data::Type::Float, 3, false);

        // Compute offsets inside the VERTEX STRUCT
        let position_offset = 0;
        let tex_coord_offset = position_offset + position_attribute.get_size();
        let color_offset = tex_coord_offset + tex_coord_attribute.get_size();
        let normal_offset = color_offset + color_attribute.get_size();

        // Allocate uninitialized data for the total size in the VBO
        self.vbo.bind();
        self.vbo.allocate_data(&vertices, Usage::StaticDraw);

        // With VAO bound, bind EBO to register it (and allocate element buffer at the same time)
        self.ebo.bind();
        self.ebo.allocate_data(&indices, Usage::StaticDraw);

        // The stride is not automatic now. Each attribute element is "sizeof(Vertex)" bytes apart from next
        let stride = mem::size_of::<Vertex>() as i32;

        // Set the pointer to the data in the VAO (notice that this offsets are for a single element)
        self.vbo.bind();
        self.vao.bind();
        self.ebo.bind();
        self.vao
            .set_attribute(0, &position_attribute, position_offset, stride);
        self.vao
            .set_attribute(1, &tex_coord_attribute, tex_coord_offset, stride);
        self.vao
            .set_attribute(2, &color_attribute, color_offset, stride);
        self.vao
            .set_attribute(3, &normal_attribute, normal_offset, stride);

        // Unbind VAO, and VBO
        self.vbo.unbind();
        self.vao.unbind();

        // Unbind EBO (when VAO is no longer bound)
        self.ebo.unbind();

        // Enable wireframe mode
        //glPolygonMode(GL_FRONT_AND_BACK, GL_LINE);

        // Enable depth buffer
        unsafe { gl::Enable(gl::DEPTH_TEST) };
        check_gl_error();
    }

    fn update(&mut self) {
        self.window.glfw_mut().poll_events();
        for (_, event) in glfw::flush_messages(&self.window.events) {
            match event {
                glfw::WindowEvent::Key(key, _, Action::Press, _) => match key {
                    Key::Escape => self.window.inner_window.set_should_close(true),
                    glfw::Key::Num0 | glfw::Key::Num1 | glfw::Key::Num2 | glfw::Key::Num3 => {
                        let i = match key {
                            glfw::Key::Num1 => 1,
                            glfw::Key::Num2 => 2,
                            glfw::Key::Num3 => 3,
                            _ => 0,
                        };

                        let mode_location =
                            unsafe { gl::GetUniformLocation(self.program.id(), c"Mode".as_ptr()) };
                        check_gl_error();
                        self.program.set_used();
                        unsafe { gl::Uniform1i(mode_location, i) };
                        check_gl_error();
                        break;
                    }
                    Key::Tab => {
                        let proj_matrix = [
                            0.0, -1.294, -0.721, -0.707, 1.83, 0.0, 0.0, 0.0, 0.0, 1.294, -0.721,
                            -0.707, 0.0, 0.0, 1.24, 1.414,
                        ];
                        let matrix_location = unsafe {
                            gl::GetUniformLocation(self.program.id(), c"Matrix".as_ptr())
                        };
                        check_gl_error();
                        self.program.set_used();
                        unsafe {
                            gl::UniformMatrix4fv(
                                matrix_location,
                                1,
                                gl::FALSE,
                                proj_matrix.as_ptr(),
                            );
                        };
                        check_gl_error();
                    }
                    _ => {}
                },
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    // make sure the viewport matches the new window dimensions; note that width and
                    // height will be significantly larger than specified on retina displays.
                    self.window.set_viewport(width, height);
                }
                _ => {}
            }
        }
        for i in 0..4 {
            let key = match i {
                1 => glfw::Key::Num1,
                2 => glfw::Key::Num2,
                3 => glfw::Key::Num3,
                _ => glfw::Key::Num0,
            };
            if self.window().inner_window.get_key(key) == Action::Press {
                let mode_location =
                    unsafe { gl::GetUniformLocation(self.program.id(), c"Mode".as_ptr()) };
                check_gl_error();
                self.program.set_used();
                unsafe { gl::Uniform1ui(mode_location, i) };
                check_gl_error();
                break;
            }
        }
        if self.window().inner_window.get_key(glfw::Key::Tab) == Action::Press {
            let proj_matrix = [
                0.0, -1.294, -0.721, -0.707, 1.83, 0.0, 0.0, 0.0, 0.0, 1.294, -0.721, -0.707, 0.0,
                0.0, 1.24, 1.414,
            ];
            let matrix_location =
                unsafe { gl::GetUniformLocation(self.program.id(), c"Matrix".as_ptr()) };
            check_gl_error();
            self.program.set_used();
            unsafe { gl::UniformMatrix4fv(matrix_location, 1, gl::FALSE, proj_matrix.as_ptr()) };
            check_gl_error();
        }
    }

    fn render(&mut self) {
        self.window.clear(0.0, 0.0, 0.0, 1.0, 1.0);

        // Set shader to be used
        self.program.set_used();

        // Bind the grid VAO
        self.vao.bind();

        // Draw the grid (m_gridX * m_gridY quads, 6 vertices per quad)
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                self.grid_x as i32 * self.grid_y as i32 * 6,
                gl::UNSIGNED_INT,
                null(),
            );
        };
        check_gl_error();

        // No need to unbind every time
        //VertexArrayObject::Unbind();
    }
}

fn get_color_from_height(height: f32) -> Vector3 {
    if height > 0.3 {
        Vector3::new(1.0, 1.0, 1.0) // Snow
    } else if height > 0.1 {
        Vector3::new(0.3, 0.3, 0.35) // Rock
    } else if height > -0.05 {
        Vector3::new(0.1, 0.4, 0.15) // Grass
    } else if height > -0.1 {
        Vector3::new(0.6, 0.5, 0.4) // Sand
    } else {
        Vector3::new(0.1, 0.1, 0.3) // Water
    }
}
fn main() {
    let app = TerrainApplication::new(1024, 1024, "TerrainDemo");
    app.run();
}

fn build_shaders() -> Program {
    let cstr = CString::new(include_str!("vert.vert")).unwrap();
    let vertex_shader = Shader::from_vert_source(&cstr).unwrap();

    let cstr = CString::new(include_str!("frag.frag")).unwrap();

    let fragment_shader = Shader::from_frag_source(&cstr).unwrap();

    Program::from_shaders(&[vertex_shader, fragment_shader]).unwrap()
}

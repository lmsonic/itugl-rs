use gl::types::GLvoid;
use glam::Vec2;
use glfw::{fail_on_errors, Context, GlfwReceiver, PWindow, WindowEvent, WindowMode};

use crate::error::check_gl_error;

#[derive(Debug)]
pub struct Window {
    pub inner_window: PWindow,
    pub events: GlfwReceiver<(f64, WindowEvent)>,
}

impl Window {
    #[must_use]
    pub fn glfw(&self) -> &glfw::Glfw {
        &self.inner_window.glfw
    }
    pub fn glfw_mut(&mut self) -> &mut glfw::Glfw {
        &mut self.inner_window.glfw
    }

    pub fn get_mouse_position(&self, normalized: bool) -> glam::Vec2 {
        let (mut x, mut y) = self.inner_window.get_cursor_pos();
        if normalized {
            let (width, height) = self.inner_window.get_size();
            x = x / width as f64 * 2.0 - 1.0;
            y = y / -height as f64 * 2.0 + 1.0;
        }
        Vec2::new(x as f32, y as f32)
    }

    pub fn enable_feature(&mut self, feature: gl::types::GLenum) {
        unsafe { gl::Enable(feature) };
    }
    pub fn disable_feature(&mut self, feature: gl::types::GLenum) {
        unsafe { gl::Disable(feature) };
    }
    pub fn set_wireframe(&mut self, enabled: bool) {
        unsafe {
            gl::PolygonMode(
                gl::FRONT_AND_BACK,
                if enabled { gl::LINE } else { gl::FILL },
            );
        };
    }
    pub fn set_vsync(&mut self, value: bool) {
        self.glfw_mut().set_swap_interval(if value {
            glfw::SwapInterval::Sync(1)
        } else {
            glfw::SwapInterval::None
        });
    }

    pub fn new(width: u32, height: u32, title: &str, window_mode: WindowMode) -> Self {
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 1));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
        let (mut inner_window, events) = glfw
            .create_window(width, height, title, window_mode)
            .expect("Failed to create GLFW window.");
        // Make the window's context current
        inner_window.make_current();
        inner_window.set_key_polling(true);

        gl::load_with(|s| glfw.get_proc_address_raw(s));
        unsafe { gl::Enable(gl::DEBUG_OUTPUT) }
        unsafe {
            gl::DebugMessageCallback(
                Some(crate::error::debug_callback),
                std::ptr::null_mut::<GLvoid>(),
            );
        };
        check_gl_error();

        Self {
            inner_window,
            events,
        }
    }
    pub fn clear(&self, r: f32, g: f32, b: f32, a: f32, depth: f64) {
        let mut mask = 0;

        unsafe { gl::ClearColor(r, g, b, a) };
        check_gl_error();
        mask |= gl::COLOR_BUFFER_BIT;

        unsafe { gl::ClearDepth(depth) };
        check_gl_error();
        mask |= gl::DEPTH_BUFFER_BIT;

        // glClearStencil(stencil);
        // mask |= GL_STENCIL_BUFFER_BIT;

        unsafe { gl::Clear(mask) };
        check_gl_error();
    }

    pub fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::ClearColor(r, g, b, a);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        };
        check_gl_error();
    }
    pub fn clear_depth(&self, depth: f64) {
        unsafe {
            gl::ClearDepth(depth);
            gl::Clear(gl::DEPTH_BUFFER_BIT);
        };
        check_gl_error();
    }

    pub fn set_viewport(&self, width: i32, height: i32) {
        unsafe { gl::Viewport(0, 0, width, height) };
        check_gl_error();
    }
}

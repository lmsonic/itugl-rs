use glfw::{fail_on_errors, Context, GlfwReceiver, PWindow, WindowEvent, WindowMode};

pub struct Window {
    pub inner_window: PWindow,
    pub events: GlfwReceiver<(f64, WindowEvent)>,
}

impl Window {
    pub fn glfw(&self) -> &glfw::Glfw {
        &self.inner_window.glfw
    }
    pub fn glfw_mut(&mut self) -> &mut glfw::Glfw {
        &mut self.inner_window.glfw
    }
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str, window_mode: WindowMode) -> Self {
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();
        let (mut inner_window, events) = glfw
            .create_window(width, height, title, window_mode)
            .expect("Failed to create GLFW window.");
        // Make the window's context current
        inner_window.make_current();
        gl::load_with(|s| glfw.get_proc_address_raw(s));
        inner_window.set_key_polling(true);
        Self {
            inner_window,
            events,
        }
    }

    pub fn clear(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::ClearColor(r, g, b, a);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        };
    }

    pub fn set_viewport(&self, width: i32, height: i32) {
        unsafe { gl::Viewport(0, 0, width, height) };
    }
}

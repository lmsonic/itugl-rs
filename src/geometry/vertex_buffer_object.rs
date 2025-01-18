use crate::{
    core::{
        buffer_object::{BufferObject, Target},
        object::{Handle, NullHandle, Object},
    },
    error::check_gl_error,
};

#[derive(Debug)]
pub struct VertexBufferObject {
    handle: Handle,
}

impl VertexBufferObject {
    #[must_use]
    pub fn new() -> Self {
        let mut handle = NullHandle;
        unsafe { gl::GenBuffers(1, &mut handle) };
        check_gl_error();
        Self { handle }
    }
}

impl Default for VertexBufferObject {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for VertexBufferObject {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.handle()) }
        check_gl_error();
    }
}
impl Object for VertexBufferObject {
    fn bind(&self) {
        unsafe { gl::BindBuffer(self.target() as u32, self.handle()) }
        check_gl_error();
    }

    fn handle(&self) -> Handle {
        self.handle
    }
}

impl BufferObject for VertexBufferObject {
    fn target(&self) -> Target {
        Target::ArrayBuffer
    }
}

use crate::core::{
    buffer_object::{BufferObject, Target},
    object::{Handle, NullHandle, Object},
};

pub struct ElementBufferObject {
    handle: Handle,
}

impl ElementBufferObject {
    pub fn new() -> Self {
        let mut handle = NullHandle;
        unsafe { gl::GenBuffers(1, &mut handle) };
        Self { handle }
    }
}

impl Default for ElementBufferObject {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for ElementBufferObject {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.handle()) }
    }
}
impl Object for ElementBufferObject {
    fn bind(&self) {
        unsafe { gl::BindBuffer(self.target() as u32, self.handle()) }
    }

    fn handle(&self) -> Handle {
        self.handle
    }
}

impl BufferObject for ElementBufferObject {
    fn target(&self) -> Target {
        Target::ElementArrayBuffer
    }
}

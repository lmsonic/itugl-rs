use crate::{
    core::{
        buffer_object::{BufferObject, Target},
        data,
        object::{Handle, NullHandle, Object},
    },
    error::check_gl_error,
};
#[derive(Debug)]
pub struct ElementBufferObject {
    handle: Handle,
}

impl ElementBufferObject {
    pub const fn is_supported_type(value: data::Type) -> bool {
        matches!(
            value,
            data::Type::UByte | data::Type::UShort | data::Type::UInt
        )
    }
}

impl ElementBufferObject {
    #[must_use]
    pub fn new() -> Self {
        let mut handle = NullHandle;
        unsafe { gl::GenBuffers(1, &mut handle) };
        check_gl_error();
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
        check_gl_error();
    }
}
impl Object for ElementBufferObject {
    fn bind(&self) {
        unsafe { gl::BindBuffer(self.target() as u32, self.handle()) }
        check_gl_error();
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

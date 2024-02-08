use std::{ffi::c_void, ptr::null_mut};

use gl::types::{GLenum, GLint, GLsizei, GLuint};

use crate::core::object::{Handle, NullHandle, Object};

use super::vertex_attribute::VertexAttribute;

#[derive(Debug)]
pub struct VertexArrayObject {
    handle: Handle,
}

impl Drop for VertexArrayObject {
    fn drop(&mut self) {
        unsafe { gl::DeleteVertexArrays(1, &self.handle) };
    }
}

impl VertexArrayObject {
    pub fn new() -> Self {
        let mut handle = NullHandle;
        unsafe { gl::GenVertexArrays(1, &mut handle) };
        Self { handle }
    }

    pub fn set_attribute(
        &self,
        location: GLuint,
        attribute: &VertexAttribute,
        offset: GLint,
        stride: GLsizei,
    ) {
        let components = attribute.components();
        let data_type = attribute.data_type() as GLenum;
        let normalized = if attribute.is_normalized() {
            gl::TRUE
        } else {
            gl::FALSE
        };

        // Compute the attribute pointer
        let mut pointer = null_mut::<u8>(); // Actual base pointer is in VBO
        pointer = pointer.wrapping_add(offset.try_into().unwrap());

        unsafe {
            gl::VertexAttribPointer(
                location,
                components,
                data_type,
                normalized,
                stride,
                pointer as *const c_void,
            );
        };
        // Set the VertexAttribute pointer in this location

        // Finally, we enable the VertexAttribute in this location
        unsafe { gl::EnableVertexAttribArray(location) };
    }
}

impl Default for VertexArrayObject {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for VertexArrayObject {
    fn bind(&self) {
        unsafe { gl::BindVertexArray(self.handle) };
    }

    fn handle(&self) -> Handle {
        self.handle
    }
}

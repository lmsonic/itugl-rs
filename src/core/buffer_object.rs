use std::{os::raw::c_void, ptr::null};

use gl::types::{GLenum, GLsizeiptr};

use crate::error::check_gl_error;

use super::object::{NullHandle, Object};
#[repr(u32)]
pub enum Target {
    // Vertex Buffer Object
    ArrayBuffer = gl::ARRAY_BUFFER,
    // Element Buffer Object
    ElementArrayBuffer = gl::ELEMENT_ARRAY_BUFFER,
    // TODO: There are more types, add them when they are supported
}
#[repr(u32)]
pub enum Usage {
    StaticDraw = gl::STATIC_DRAW,
    StaticRead = gl::STATIC_READ,
    StaticCopy = gl::STATIC_COPY,
    DynamicDraw = gl::DYNAMIC_DRAW,
    DynamicRead = gl::DYNAMIC_READ,
    DynamicCopy = gl::DYNAMIC_COPY,
    StreamDraw = gl::STREAM_DRAW,
    StreamRead = gl::STREAM_READ,
    StreamCopy = gl::STREAM_COPY,
}
pub trait BufferObject: Object {
    fn unbind(&self) {
        unsafe { gl::BindBuffer(self.target() as u32, NullHandle) }
        check_gl_error();
    }
    fn target(&self) -> Target;
    fn allocate_data<T>(&self, data: &[T], usage: Usage) {
        unsafe {
            gl::BufferData(
                self.target() as GLenum,
                std::mem::size_of_val(data) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                usage as GLenum,
            );
        }
        check_gl_error();
    }
    fn reserve_data(&self, size: GLsizeiptr, usage: Usage) {
        unsafe {
            gl::BufferData(
                self.target() as GLenum,
                size,
                null() as *const c_void,
                usage as GLenum,
            );
        }
        check_gl_error();
    }
    fn update_data<T>(&self, data: &[T], offset: GLsizeiptr) {
        unsafe {
            gl::BufferSubData(
                self.target() as GLenum,
                offset,
                std::mem::size_of_val(data) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
            );
        }
        check_gl_error();
    }
}

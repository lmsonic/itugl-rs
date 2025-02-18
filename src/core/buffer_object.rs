use std::{os::raw::c_void, ptr::null};

use gl::types::{GLenum, GLsizeiptr};

use crate::error::check_gl_error;

use super::object::{NullHandle, Object};
#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum Target {
    // Vertex Buffer Object
    ArrayBuffer = gl::ARRAY_BUFFER,
    // Element Buffer Object
    ElementArrayBuffer = gl::ELEMENT_ARRAY_BUFFER,
    // TODO: There are more types, add them when they are supported
}
#[repr(u32)]
#[derive(Clone, Copy, Debug)]
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
        self.bind();
        unsafe {
            gl::BufferData(
                self.target() as GLenum,
                std::mem::size_of_val(data) as gl::types::GLsizeiptr,
                data.as_ptr().cast::<gl::types::GLvoid>(),
                usage as GLenum,
            );
        }
        self.unbind();
        check_gl_error();
    }
    fn reserve_data<T>(&self, size: usize, usage: Usage) {
        self.bind();
        unsafe {
            gl::BufferData(
                self.target() as GLenum,
                (size * size_of::<T>()) as gl::types::GLsizeiptr,
                null::<c_void>(),
                usage as GLenum,
            );
        }
        self.unbind();
        check_gl_error();
    }
    fn update_data<T>(&self, data: &[T], offset: GLsizeiptr) {
        self.bind();
        unsafe {
            gl::BufferSubData(
                self.target() as GLenum,
                offset,
                std::mem::size_of_val(data) as gl::types::GLsizeiptr,
                data.as_ptr().cast::<gl::types::GLvoid>(),
            );
        }
        self.unbind();
        check_gl_error();
    }
}

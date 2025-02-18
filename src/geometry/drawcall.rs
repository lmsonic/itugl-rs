use std::ffi::c_void;

use gl::types::{GLenum, GLint, GLsizei};

use crate::{
    core::data, error::check_gl_error, geometry::element_buffer_object::ElementBufferObject,
};

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum Primitive {
    Points = gl::POINTS,
    Lines = gl::LINES,
    LineStrip = gl::LINE_STRIP,
    LineLoop = gl::LINE_LOOP,
    LinesAdjacency = gl::LINES_ADJACENCY,
    LineStripAdjacency = gl::LINE_STRIP_ADJACENCY,
    Triangles = gl::TRIANGLES,
    TriangleStrip = gl::TRIANGLE_STRIP,
    TriangleFan = gl::TRIANGLE_FAN,
    TrianglesAdjacency = gl::TRIANGLES_ADJACENCY,
    TriangleStripAdjacency = gl::TRIANGLE_STRIP_ADJACENCY,
    Patches = gl::PATCHES,
}

#[derive(Clone, Copy, Debug)]
pub struct DrawCall {
    primitive: Primitive,
    first: GLint,
    count: GLsizei,
    index_type: data::Type,
}

impl DrawCall {
    pub const fn new(primitive: Primitive) -> Self {
        Self {
            primitive,
            first: 0,
            count: 1,
            index_type: data::Type::None,
        }
    }
    pub const fn first(value: Self, first: GLint) -> Self {
        Self { first, ..value }
    }
    pub const fn count(value: Self, count: GLsizei) -> Self {
        Self { count, ..value }
    }
    pub const fn index_type(value: Self, index_type: data::Type) -> Self {
        assert!(ElementBufferObject::is_supported_type(index_type));
        Self {
            index_type,
            ..value
        }
    }

    pub fn draw(&self) {
        let primitive = self.primitive as GLenum;
        if self.index_type == data::Type::None {
            // If no EBO is present, use glDrawArrays
            unsafe { gl::DrawArrays(primitive, self.first, self.count) }
            check_gl_error();
        } else {
            let base_pointer: *const c_void = std::ptr::null();
            let index_type = self.index_type as GLenum;
            unsafe {
                gl::DrawElements(
                    primitive,
                    self.count,
                    index_type,
                    base_pointer.wrapping_byte_add(self.first as usize),
                )
            };
        }
    }
}

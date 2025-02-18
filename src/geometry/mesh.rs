use crate::core::buffer_object::{BufferObject, Usage};

use super::{
    element_buffer_object::ElementBufferObject, vertex_array_object::VertexArrayObject,
    vertex_buffer_object::VertexBufferObject,
};

#[derive(Debug)]
pub struct Mesh {
    vbos: Vec<VertexBufferObject>,
    ebos: Vec<ElementBufferObject>,
    abos: Vec<VertexArrayObject>,
}
impl Mesh {
    pub fn add_vertex_data<T>(&mut self, vertices: &[T]) {
        let vbo = VertexBufferObject::new();
        vbo.allocate_data(vertices, Usage::StaticDraw);
        self.vbos.push(vbo);
    }
    pub fn add_element_data<T>(&mut self, indices: &[T]) {
        let ebo = ElementBufferObject::new();
        ebo.allocate_data(indices, Usage::StaticDraw);
        self.ebos.push(ebo);
    }
}

use gl::types::GLint;

use crate::core::data::{self, Type};

#[derive(Clone, Copy, Debug)]
pub struct VertexAttribute {
    data_type: data::Type,
    components: GLint,
    normalized: bool,
}

impl VertexAttribute {
    pub fn new(data_type: data::Type, components: GLint, normalized: bool) -> Self {
        Self {
            data_type,
            components,
            normalized,
        }
    }
    pub fn get_size(&self) -> GLint {
        self.data_type.get_size() * self.components
    }

    pub fn data_type(&self) -> Type {
        self.data_type
    }

    pub fn components(&self) -> GLint {
        self.components
    }

    pub fn is_normalized(&self) -> bool {
        self.normalized
    }
}

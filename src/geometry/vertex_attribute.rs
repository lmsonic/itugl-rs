use gl::types::{GLint, GLsizei};

use crate::core::data::{self, Type};
#[derive(Clone, Copy, Debug)]
pub struct Layout {
    attribute: VertexAttribute,
    offset: GLint,
    stride: GLsizei,
}

impl Layout {
    pub const fn new(attribute: VertexAttribute, offset: GLint, stride: GLsizei) -> Self {
        Self {
            attribute,
            offset,
            stride,
        }
    }

    pub const fn attribute(&self) -> VertexAttribute {
        self.attribute
    }

    pub const fn offset(&self) -> i32 {
        self.offset
    }

    pub const fn stride(&self) -> i32 {
        self.stride
    }
}

#[derive(Clone, Copy, Debug)]
pub struct VertexAttribute {
    data_type: data::Type,
    components: GLint,
    normalized: bool,
}

impl VertexAttribute {
    #[must_use]
    pub const fn new(data_type: data::Type, components: GLint, normalized: bool) -> Self {
        Self {
            data_type,
            components,
            normalized,
        }
    }
    #[must_use]
    pub const fn get_size(&self) -> GLint {
        self.data_type.get_size() * self.components
    }

    #[must_use]
    pub const fn data_type(&self) -> Type {
        self.data_type
    }

    #[must_use]
    pub const fn components(&self) -> GLint {
        self.components
    }

    #[must_use]
    pub const fn is_normalized(&self) -> bool {
        self.normalized
    }
    #[must_use]
    pub fn is_floating_point(&self) -> bool {
        self.data_type == data::Type::Float
            || self.data_type == data::Type::Double
            || self.data_type == data::Type::Fixed
    }
}

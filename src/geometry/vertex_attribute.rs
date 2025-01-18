use gl::types::GLint;

use crate::core::data::{self, Type};

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
}

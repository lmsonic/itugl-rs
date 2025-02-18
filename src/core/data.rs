use gl::types::GLint;

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Type {
    None = gl::NONE,
    Float = gl::FLOAT,
    Fixed = gl::FIXED,
    Half = gl::HALF_FLOAT,
    Double = gl::DOUBLE,
    Byte = gl::BYTE,
    UByte = gl::UNSIGNED_BYTE,
    Short = gl::SHORT,
    UShort = gl::UNSIGNED_SHORT,
    Int = gl::INT,
    UInt = gl::UNSIGNED_INT,
    // And more...
}
impl Type {
    pub(crate) const fn get_size(self) -> GLint {
        match self {
            Self::Byte | Self::UByte => 1,
            Self::Short | Self::UShort | Self::Half => 2,
            Self::Float => 4,
            Self::Int | Self::UInt => 4,
            Self::Fixed => 2,
            Self::Double => 8,
            Self::None => 0,
        }
    }
}

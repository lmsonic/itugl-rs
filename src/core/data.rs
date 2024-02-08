use gl::types::GLint;

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
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
    pub(crate) fn get_size(&self) -> GLint {
        match self {
            Type::Byte | Type::UByte => 1,
            Type::Short | Type::UShort | Type::Half => 2,
            Type::Double => 8,
            _ => 4,
        }
    }
}

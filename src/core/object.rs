use gl::types::GLuint;

pub type Handle = GLuint;
pub const NullHandle: Handle = 0;

pub trait Object {
    fn bind(&self);
    fn handle(&self) -> Handle;
}

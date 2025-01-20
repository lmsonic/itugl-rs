use gl::types::GLuint;

pub type Handle = GLuint;
#[allow(non_upper_case_globals)]
pub const NullHandle: Handle = 0;

pub trait Object {
    fn bind(&self);
    fn handle(&self) -> Handle;
}

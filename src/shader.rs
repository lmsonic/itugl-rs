use std::ffi::{CStr, CString};

use crate::error::check_gl_error;

pub type Location = gl::types::GLint;

#[derive(Debug)]
pub struct Shader {
    id: gl::types::GLuint,
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
            check_gl_error();
        }
    }
}

impl Shader {
    pub fn from_source(source: &CStr, kind: gl::types::GLuint) -> Result<Self, String> {
        let id = shader_from_source(source, kind)?;
        Ok(Self { id })
    }
    pub fn from_vert_source(source: &CStr) -> Result<Self, String> {
        Self::from_source(source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(source: &CStr) -> Result<Self, String> {
        Self::from_source(source, gl::FRAGMENT_SHADER)
    }

    #[must_use]
    pub const fn id(&self) -> u32 {
        self.id
    }
}

fn shader_from_source(source: &CStr, kind: gl::types::GLuint) -> Result<gl::types::GLuint, String> {
    let id = unsafe { gl::CreateShader(kind) };
    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }
    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }
    check_gl_error();
    if success == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }
        // convert buffer to CString
        let error: CString = create_whitespace_cstring_with_len(len as usize);
        unsafe {
            gl::GetShaderInfoLog(id, len, std::ptr::null_mut(), error.as_ptr().cast_mut());
        }
        check_gl_error();

        return Err(error.to_string_lossy().into_owned());
    }

    Ok(id)
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend(std::iter::once(&b' ').cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}

#[derive(Debug)]
pub struct Program {
    id: gl::types::GLuint,
}
impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
            check_gl_error();
        }
    }
}

impl Program {
    pub fn from_shaders(shaders: &[Shader]) -> Result<Self, String> {
        let program_id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe {
                gl::AttachShader(program_id, shader.id());
            }
            check_gl_error();
        }

        unsafe {
            gl::LinkProgram(program_id);
        }
        check_gl_error();

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }
        check_gl_error();

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }
            check_gl_error();

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr().cast_mut(),
                );
                check_gl_error();
            }

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe {
                gl::DetachShader(program_id, shader.id());
            }
            check_gl_error();
        }

        Ok(Self { id: program_id })
    }
    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
        check_gl_error();
    }

    #[must_use]
    pub const fn id(&self) -> u32 {
        self.id
    }

    pub fn get_uniform_location(&self, name: &CStr) -> Location {
        unsafe { gl::GetUniformLocation(self.id, name.as_ptr()) }
    }

    pub fn get_attribute_location(&self, name: &CStr) -> Location {
        unsafe { gl::GetAttribLocation(self.id, name.as_ptr()) }
    }
}

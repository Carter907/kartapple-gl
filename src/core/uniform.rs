use std::any::Any;
use std::ffi::CString;
use gl::types::{GLint, GLuint};
use crate::core::gl_var_type::GLvartype;

pub struct Uniform {}

impl Uniform {
    pub unsafe fn locate_uniform(program_id: GLuint, name: &str) -> GLint {
        let name = CString::new(name).unwrap();
        let uniform_id = gl::GetUniformLocation(program_id, name.as_ptr());
        match uniform_id {
            -1 => {
                panic!("could not find uniform")
            }
            _ => {
                uniform_id
            }
        }
    }


}

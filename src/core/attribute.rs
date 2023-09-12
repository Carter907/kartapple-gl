use std::ffi::CString;
use std::{mem, ptr};
use std::collections::HashMap;
use std::mem::size_of_val;
use std::os::raw::c_void;
use gl::types::{GLchar, GLsizeiptr, GLuint};
use crate::core::gl_var_type::GLvartype;


/// used store Attribute data and provides helper methods for initializing buffers.
pub struct Attribute;
impl Attribute {


    pub unsafe fn init(data: &[f32]) {
        let mut bao = GLuint::from(1u32);
        gl::GenBuffers(1, &mut bao);
        gl::BindBuffer(gl::ARRAY_BUFFER, bao);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (size_of_val(data)) as GLsizeiptr,
            data.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );
    }
    pub unsafe fn locate_attribute(program: GLuint, name: &str, kind: GLvartype) {
        let name = CString::new(name).unwrap();
        let attrib = gl::GetAttribLocation(program,
                                           name.as_ptr(),
        );
        let attrib = match attrib {
            -1 => {
                panic!("attrib is not found");
            }
            _ => {
                GLuint::try_from(attrib).unwrap()
            }
        };


        gl::VertexAttribPointer(attrib, GLvartype::get_type_size(kind), gl::FLOAT,
                                0, 0, ptr::null());

        gl::EnableVertexAttribArray(attrib);
    }
}
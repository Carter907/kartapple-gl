use gl::types::{GLsizeiptr, GLuint};
use std::ffi::c_void;
use std::mem;

/// Index utility class for setting the index ordering of attribute data

pub struct Index {}

impl Index {
    pub unsafe fn init_f32(ebo: &mut GLuint, indices: &[f32]) {
        gl::GenBuffers(1, ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, *ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            mem::size_of_val(indices) as GLsizeiptr,
            indices.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );
    }
    pub unsafe fn init_i32(ebo: &mut GLuint, indices: &[i32]) {
        gl::GenBuffers(1, ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, *ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            mem::size_of_val(indices) as GLsizeiptr,
            indices.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );
    }
}

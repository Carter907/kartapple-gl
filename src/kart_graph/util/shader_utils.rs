use std::ffi::CString;
use std::fs::*;
use std::io::Read;
use std::ptr;
use gl::types::{GLenum, GLuint};

pub struct ShaderUtils {}

impl ShaderUtils {
    pub fn get_shader_code(path: &str) -> String {
        let mut file = match File::open(path) {
            Ok(file) => {
                file
            }
            Err(e) => {
                panic!("{e:?}");
            }
        };
        let mut contents = String::new();

        file.read_to_string(&mut contents)
            .expect("Failed to read fail contents!");

        contents
    }
    pub unsafe fn get_shader(kind: GLenum, shader_code: &String) -> GLuint {
        let shader_id = gl::CreateShader(kind);
        let c_str_shader = CString::new(shader_code.as_bytes()).unwrap();
        gl::ShaderSource(shader_id, 1, &c_str_shader.as_ptr(), ptr::null());
        gl::CompileShader(shader_id);

        shader_id
    }
}
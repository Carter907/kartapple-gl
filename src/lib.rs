
#![allow(dead_code)]
#![allow(unused)]


pub mod core;
pub mod util;




#[cfg(test)]
mod tests {

    extern crate glfw;
    extern crate gl;

    use std::cmp::Ordering;
    use std::ffi::CString;
    use std::os::raw::c_void;
    use std::{mem, ptr};
    use std::time::SystemTime;
    use cgmath::num_traits::{FromPrimitive, ToPrimitive};
    use gl::types::{GLchar, GLfloat, GLsizeiptr, GLuint};
    use crate::core::attribute::Attribute;
    use crate::core::gl_var_type::GLvartype;
    use crate::core::kukari_app::KukariApp;
    use crate::core::uniform::Uniform;
    use crate::util::program_utils::ProgramUtils;
    use crate::util::shader_utils::ShaderUtils;


    #[test]
    fn it_works() {
        unsafe {
            let mut application = KukariApp::new(700, 500, "window");
            application.init();

            let mut program: GLuint = GLuint::from(1u32);
            let vert_shader_code = ShaderUtils::get_shader_code("res/vert.glsl");
            let frag_shader_code = ShaderUtils::get_shader_code("res/frag.glsl");
            program = ProgramUtils::create_program(&vert_shader_code, &frag_shader_code);
            application.set_program(program);
            let mut vao = GLuint::from(1u32);

            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            let vertices = [
                -0.5, -0.5, 0.0,
                0.5, -0.5, 0.0,
                0.0, 0.5, 0.0
            ];


            Attribute::init(&vertices);
            Attribute::locate_attribute(program, "position", GLvartype::Vec3);

            let vert_colors = [
                1.0, 0.0, 0.0,
                0.0, 1.0, 0.0,
                0.0, 0.0, 1.0,
            ];
            Attribute::init(&vert_colors);
            Attribute::locate_attribute(program, "vertColor", GLvartype::Vec3);

            let translX = Uniform::locate_uniform(program, "translX");

            let mut delta_time = 0;
            let mut tot_time = 0;
            let mut time_elapsed = SystemTime::now();
            while !application.should_close() {
                gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                gl::UseProgram(application.program.unwrap());

                gl::Uniform1f(translX, 0.5f32);

                gl::DrawArrays(gl::TRIANGLES, 0, 3);


                application.window.update();
                application.window.process_events();
                delta_time = time_elapsed.elapsed().unwrap().as_millis();
                tot_time += delta_time;
                time_elapsed = SystemTime::now();
                println!("{delta_time:?}");
            }
        }
    }
}

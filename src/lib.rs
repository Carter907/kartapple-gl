
pub mod core;
pub mod util;

pub extern crate glfw;
pub extern crate gl;
pub extern crate cgmath;

#[cfg(test)]
mod test {

    use std::f32::consts::PI;
    use crate::cgmath::{PerspectiveFov, Rad};
    use crate::{gl, glfw};
    use crate::core::scaffold::{AppScaffold};
    use crate::core::application::*;
    use crate::core::attribute::Attribute;
    use crate::core::camera::Camera;
    use crate::core::gl_var_type::GLvartype;
    use gl::types::GLuint;
    use glfw::{Action, Key, Modifiers, Scancode};
    use crate::util::program_utils::ProgramUtils;

    #[test]
    fn three_d_spinning_cube() {
        unsafe {
            KartApple::start(
                PrismGL::new(),
                700,
                500,
                "window"
            );
        }
    }


    pub struct PrismGL {
        cam: Option<Camera>,
        degrees: f32
    }
    impl PrismGL {
        pub fn new() -> PrismGL {
            PrismGL {
                cam: None,
                degrees: 0f32,
            }
        }
    }
    impl AppScaffold for PrismGL {


        unsafe fn on_init(&mut self, app: &mut KartApple) {
            gl::Enable(gl::DEPTH_TEST);


            let mut program = GLuint::from(1u32);
            let vert_code = include_str!("../shaders/vert.glsl").to_string();
            let frag_code = include_str!("../shaders/frag.glsl").to_string();

            program = ProgramUtils::create_program(&vert_code, &frag_code);

            app.set_program(program);

            let mut vao = GLuint::from(1u32);
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            let side_length = 0.75f32;

            let vertices = [
                // side 1
                -side_length, -side_length, -side_length,
                side_length, -side_length, -side_length,
                side_length, side_length, -side_length,
                -side_length, side_length, -side_length,

                // side 2
                -side_length, side_length, side_length,
                -side_length, -side_length, side_length,
                -side_length, -side_length, -side_length,
                -side_length, side_length, -side_length,

                // side 3
                -side_length, side_length, -side_length,
                -side_length, side_length, side_length,
                side_length, side_length, side_length,
                side_length, side_length, -side_length,


                // side 4
                side_length, -side_length, -side_length,
                side_length, -side_length, side_length,
                side_length, side_length, side_length,
                side_length, side_length, -side_length,

                // side 5
                -side_length, side_length, side_length,
                -side_length, -side_length, side_length,
                side_length, -side_length, side_length,
                side_length, side_length, side_length,

                // side 6
                -side_length, -side_length, -side_length,
                side_length, -side_length, -side_length,
                side_length, -side_length, side_length,
                -side_length, -side_length, side_length
            ];

            Attribute::init(&vertices);
            Attribute::locate_attribute(program, "pos", GLvartype::Vec3);

            let vert_colors = [
                // side 1

                0.0, 1.0, 1.0,
                0.0, 1.0, 1.0,
                0.0, 1.0, 1.0,
                0.0, 1.0, 1.0,

                // side 2

                1.0, 1.0, 0.0,
                1.0, 1.0, 0.0,
                1.0, 1.0, 0.0,
                1.0, 1.0, 0.0,

                // side 3

                1.0, 0.0, 1.0,
                1.0, 0.0, 1.0,
                1.0, 0.0, 1.0,
                1.0, 0.0, 1.0,

                // side 4

                1.0, 0.0, 0.0,
                1.0, 0.0, 0.0,
                1.0, 0.0, 0.0,
                1.0, 0.0, 0.0,

                // side 5

                0.0, 1.0, 0.0,
                0.0, 1.0, 0.0,
                0.0, 1.0, 0.0,
                0.0, 1.0, 0.0,

                // side 6

                1.0, 1.0, 1.0,
                1.0, 1.0, 1.0,
                1.0, 1.0, 1.0,
                1.0, 1.0, 1.0,
            ];

            Attribute::init(&vert_colors);
            Attribute::locate_attribute(program, "vertColor", GLvartype::Vec3);


            let mut cam = Camera::new(program, "model", "view", "proj");

            let fov = 90f32;
            cam.projection(PerspectiveFov {
                fovy: Rad((PI / 180f32) * fov),
                aspect: (app.window.width as f32 / app.window.height as f32),
                near: (0.1f32),
                far: (100f32),
            });
            cam.translate_view_z(-2.50f32);
            cam.rotate_view_x(40.0);
            self.cam = Some(cam);
            self.degrees = 60f32;

        }

        unsafe fn on_loop(&mut self, app: &mut KartApple) {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::UseProgram(app.program.unwrap());
            self.cam.as_mut().unwrap().rotate_model_y(app.delta.value as f32 * self.degrees / 1000000f32);
            self.cam.as_mut().unwrap().update();


            gl::DrawArrays(gl::QUADS, 0, 24);
        }

        unsafe fn on_key(&mut self, key: Key, scancode: Scancode, action: Action, modifiers: Modifiers, app: &mut KartApple) {
        }

        unsafe fn on_resize(&mut self, width: i32, height: i32, app: &mut KartApple) {
            gl::Viewport(0,0, width, height);
        }

        unsafe fn on_clean(&mut self, app: &mut KartApple) {
            // gl::DeleteVertexArrays(1, &vao);
            // gl::DeleteBuffers(1, &vao);
            // gl::DeleteProgram(app.program.unwrap());
            unsafe { glfw::ffi::glfwTerminate(); }
        }
    }
}
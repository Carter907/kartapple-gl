pub mod core;
pub mod util;

pub extern crate cgmath;
pub extern crate gl;
pub extern crate glfw;

#[cfg(test)]
mod test {

    use crate::cgmath::{PerspectiveFov, Rad};
    use crate::core::application::*;
    use crate::core::attribute::Attribute;
    use crate::core::camera::Camera;
    use crate::core::gl_var_type::GLvartype;
    use crate::core::scaffold::AppScaffold;
    use crate::util::program_utils::ProgramUtils;
    use crate::{gl, glfw};
    use gl::types::GLuint;
    use glfw::{Action, Key, Modifiers, MouseButton, Scancode};
    use std::f32::consts::PI;

    #[test]
    fn three_d_spinning_cube() {
        unsafe {
            KartApple::start(PrismGL::new(), 700, 500, "window");
        }
    }

    pub struct PrismGL {
        cam: Option<Camera>,
        degrees: f32,
        vao: GLuint,
        program: GLuint
    }
    impl PrismGL {
        pub fn new() -> PrismGL {
            PrismGL {
                cam: None,
                degrees: 0f32,
                vao: GLuint::from(1u32),
                program: GLuint::from(1u32)
            }
        }
    }
    impl AppScaffold for PrismGL {
        unsafe fn on_init(&mut self, app: &mut KartApple) {
            gl::Enable(gl::DEPTH_TEST);

            let vert_code = include_str!("../shaders/vert.glsl").to_string();
            let frag_code = include_str!("../shaders/frag.glsl").to_string();

            self.program = ProgramUtils::create_program(&vert_code, &frag_code);

            let mut vao = GLuint::from(1u32);
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            self.vao = vao;
            let side_length = 0.75f32;

            let vertices = [
                // side 1
                -side_length,
                -side_length,
                -side_length,
                side_length,
                -side_length,
                -side_length,
                side_length,
                side_length,
                -side_length,
                -side_length,
                side_length,
                -side_length,
                // side 2
                -side_length,
                side_length,
                side_length,
                -side_length,
                -side_length,
                side_length,
                -side_length,
                -side_length,
                -side_length,
                -side_length,
                side_length,
                -side_length,
                // side 3
                -side_length,
                side_length,
                -side_length,
                -side_length,
                side_length,
                side_length,
                side_length,
                side_length,
                side_length,
                side_length,
                side_length,
                -side_length,
                // side 4
                side_length,
                -side_length,
                -side_length,
                side_length,
                -side_length,
                side_length,
                side_length,
                side_length,
                side_length,
                side_length,
                side_length,
                -side_length,
                // side 5
                -side_length,
                side_length,
                side_length,
                -side_length,
                -side_length,
                side_length,
                side_length,
                -side_length,
                side_length,
                side_length,
                side_length,
                side_length,
                // side 6
                -side_length,
                -side_length,
                -side_length,
                side_length,
                -side_length,
                -side_length,
                side_length,
                -side_length,
                side_length,
                -side_length,
                -side_length,
                side_length,
            ];

            Attribute::init(&vertices);
            Attribute::locate_attribute(self.program, "pos", GLvartype::Vec3);

            let vert_colors = [
                // side 1
                0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, // side 2
                1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, // side 3
                1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, // side 4
                1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, // side 5
                0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, // side 6
                1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
            ];

            Attribute::init(&vert_colors);
            Attribute::locate_attribute(self.program, "vertColor", GLvartype::Vec3);

            let mut cam = Camera::new(self.program, "model", "view", "proj");

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
            gl::UseProgram(self.program);
            self.cam
                .as_mut()
                .unwrap()
                .rotate_model_y(app.delta.value as f32 * self.degrees / 1000000f32);
            self.cam.as_mut().unwrap().update();

            gl::DrawArrays(gl::QUADS, 0, 24);
        }

        unsafe fn on_key(&mut self, key: Key, scancode: Scancode, action: Action, modifiers: Modifiers, app: &mut KartApple) {
            println!("key: {key:?}");

            match key {
                Key::W => {
                    self.degrees += 1.0;
                }
                _ => {}
            }
        }

        unsafe fn on_mouse(&mut self, button: MouseButton, action: Action, modifiers: Modifiers, app: &mut KartApple) {
        }

        unsafe fn on_resize(&mut self, width: i32, height: i32, app: &mut KartApple) {
            gl::Viewport(0, 0, width, height);
        }

        unsafe fn on_clean(&mut self, app: &mut KartApple) {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vao);
            gl::DeleteProgram(self.program);

        }
    }
}

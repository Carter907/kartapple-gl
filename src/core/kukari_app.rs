use gl::types::GLuint;
use crate::core::window::Window;


pub struct KukariApp {
    pub window: Window,
    pub program: Option<GLuint>,
}

impl KukariApp {
    pub fn should_close(&mut self) -> bool {
        self.window.should_close()
    }
    pub fn new(width: u32, height: u32, title: &str) -> KukariApp {
        KukariApp {
            window: Window::new(width, height, title),
            program: Option::None,
        }
    }
    pub fn init(&mut self) {
        self.window.init_gl();
    }
    pub fn set_program(&mut self, program_id: GLuint) {
        self.program = Some(program_id);
    }
    pub fn start(&mut self) {


        // Create a windowed mode window and its OpenGL context
        // Make the window's context current


        // Loop until the user closes the window
        while !self.window.should_close() {
            // Swap front and back buffers
            // and poll events


        }
        self.shutdown();
    }

    fn shutdown(&self) {}
}
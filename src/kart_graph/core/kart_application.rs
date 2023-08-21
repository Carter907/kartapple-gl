use std::alloc::System;
use std::time::SystemTime;
use gl::types::GLuint;
use crate::kart_graph::core::window::Window;

pub struct DeltaTime {
    total_time: u128,
    pub value: u128,
    at_iteration: SystemTime,
}
impl DeltaTime {
    pub(crate) fn calc_time(&mut self) {

        self.value = self.at_iteration.elapsed().unwrap().as_millis();
        self.total_time += self.value;
        self.at_iteration = SystemTime::now();

    }
}

pub struct Kartappl {
    pub window: Window,
    pub program: Option<GLuint>,
    pub delta: DeltaTime,
}

impl Kartappl {
    pub fn should_close(&mut self) -> bool {
        self.window.should_close()
    }
    pub fn new(width: u32, height: u32, title: &str) -> Kartappl {
        Kartappl {
            window: Window::new(width, height, title),
            program: None,
            delta: DeltaTime {
                total_time: 0,
                value: 0,
                at_iteration: SystemTime::now(),
            }
        }
    }
    pub fn init(&mut self) {
        self.window.init_gl();
    }
    pub fn set_program(&mut self, program_id: GLuint) {
        self.program = Some(program_id);
    }
    pub fn update(&mut self) {
        self.window.update();

    }
    pub fn calc_time(&mut self) {
        self.delta.calc_time();
    }
    fn shutdown(&self) {}
}
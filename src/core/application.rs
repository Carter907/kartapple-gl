use std::alloc::System;
use std::io::Read;
use std::ops::Deref;
use std::time::SystemTime;
use gl::types::GLuint;
use glfw::{Action, flush_messages, Key, WindowEvent};
use crate::core::key_handle::DefaultKeyHandler;
use crate::core::scaffold::AppScaffold;
use crate::core::window::Window;

/// used to calculate and store helpful time calculations such as delta time and total time since game loop
pub struct DeltaTime {
    total_time: u128,
    pub value: u128,
    at_iteration: SystemTime,
}

impl DeltaTime {
    pub(crate) fn calc_time(&mut self) {
        self.value = self.at_iteration.elapsed().unwrap().as_micros();
        self.total_time += self.value;
        self.at_iteration = SystemTime::now();
    }
}

pub struct KartApple {
    pub window: Window<DefaultKeyHandler>,
    pub program: Option<GLuint>,
    pub delta: DeltaTime,
}

impl KartApple {
    pub fn should_close(&mut self) -> bool {
        self.window.should_close()
    }
    pub unsafe fn start(mut scaffold: impl AppScaffold, width: u32, height: u32, title: &str) {
        let mut kartapple = KartApple {
            window: Window::new(width, height, title),
            program: None,
            delta: DeltaTime {
                total_time: 0,
                value: 0,
                at_iteration: SystemTime::now(),
            },
        };

        kartapple.init(&mut scaffold);

        while !kartapple.should_close() {
            kartapple.update(&mut scaffold);
        }
        kartapple.shutdown(&mut scaffold);
    }
    unsafe fn init(&mut self, scaffold: &mut impl AppScaffold) {
        self.window.init_gl();
        scaffold.on_init(self);
    }
    pub fn set_program(&mut self, program_id: GLuint) {
        self.program = Some(program_id);
    }
    pub unsafe fn update(&mut self, scaffold: &mut impl AppScaffold) {
        scaffold.on_loop(self);
        self.window.update_screen();
        self.delta.calc_time();
        self.process_events(scaffold);
    }

    unsafe fn process_events(&mut self, scaffold: &mut impl AppScaffold) {
        let mut events: Vec<(f64, WindowEvent)> = flush_messages(&self.window.events)
            .into_iter()
            .collect();

        for (_, event) in events {
            match event {
                WindowEvent::Key(key, scancode, action, modifiers) => {
                    scaffold.on_key(key, scancode, action, modifiers, self)
                }
                WindowEvent::FramebufferSize(width, height) => {
                    scaffold.on_resize(width, height, self);
                }
                _ => {}
            }
        }
    }
    unsafe fn shutdown(&mut self, scaffold: &mut impl AppScaffold) {
        scaffold.on_clean(self);
    }
}
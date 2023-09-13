use crate::core::key_handle::DefaultKeyHandler;
use crate::core::scaffold::AppScaffold;
use crate::core::window::Window;
use gl::types::GLuint;
use glfw::{flush_messages, Action, Key, WindowEvent, Context};
use std::alloc::System;
use std::io::Read;
use std::ops::Deref;
use std::time::SystemTime;

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
    /// Wrapper method that tests if the app's window should close.
    pub fn should_close(&mut self) -> bool {
        self.window.should_close()
    }
    /// Creates a new KartApple application, the start method takes in a Scaffold as the blueprint of your opengl app.
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
    /// First to be called in the start function, this method initializes gl and calls the scaffolds on_init method.
    unsafe fn init(&mut self, scaffold: &mut impl AppScaffold) {


        self.window.glfw_win.set_key_polling(true);
        self.window.glfw_win.set_framebuffer_size_polling(true);
        self.window.glfw_win.make_current();

        self.window.init_gl();

        scaffold.on_init(self);
    }
    /// General method which is called on each iteration of the render loop. This in turn calls the scaffold's on_loop method as well
    /// as swapping the frame buffers and calculating the delta time.
    pub unsafe fn update(&mut self, scaffold: &mut impl AppScaffold) {
        scaffold.on_loop(self);
        self.window.update_screen();
        self.delta.calc_time();
        self.process_events(scaffold);
    }
    /// Iterates over the window events and invokes the appropriate Scaffold method depending on the WindowEvent variant.
    unsafe fn process_events(&mut self, scaffold: &mut impl AppScaffold) {
        let mut events: Vec<(f64, WindowEvent)> =
            flush_messages(&self.window.events).into_iter().collect();

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
    /// Wrapper method for AppScaffold's on_clean. For future implementation of abstracted clean up.
    unsafe fn shutdown(&mut self, scaffold: &mut impl AppScaffold) {
        scaffold.on_clean(self);
    }
}

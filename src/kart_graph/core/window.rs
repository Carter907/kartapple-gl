extern crate gl;
extern crate glfw;

use std::sync::mpsc::Receiver;
use glfw::*;

pub struct Window {
    pub glfw: Glfw,
    pub window: glfw::Window,
    pub events: Receiver<(f64, WindowEvent)>,
    pub width: u32,
    pub height: u32,
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Window {
        let mut glfw = init(FAIL_ON_ERRORS).unwrap();
        let (mut window, events) = glfw.create_window(
            width, height, title, WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);
        Window {
            glfw,
            window,
            events,
            width,
            height,
        }
    }


    pub fn make_current(&mut self) {
        self.window.make_current();
    }
    pub fn update(&mut self) {
        self.glfw.poll_events();
        self.window.swap_buffers();
    }

    pub fn init_gl(&mut self) {
        self.window.make_current();
        gl::load_with(|s| self.window.get_proc_address(s) as *const _);
    }

    pub fn should_close(&mut self) -> bool {
        self.window.should_close()
    }
    pub fn process_events(&mut self) {
        for (_, event) in flush_messages(&self.events) {
            println!("{:?}", event);
            match event {
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window.set_should_close(true);
                },
                WindowEvent::FramebufferSize(width, height) => {
                    unsafe { gl::Viewport(0,0,width, height); }
                },
                _ => {}
            }
        }
    }
}
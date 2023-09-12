extern crate gl;
extern crate glfw;

use crate::core::key_handle::{DefaultKeyHandler, KeyHandler};
use std::sync::mpsc::Receiver;
use glfw::{Context, FAIL_ON_ERRORS, Glfw, WindowEvent, WindowMode};

/// utility class for handling glfw windows and events.
#[derive(Debug)]
pub struct Window<T: KeyHandler> {
    pub glfw: Glfw,
    pub glfw_win: glfw::Window,
    pub events: Receiver<(f64, WindowEvent)>,
    pub key_handler: Option<T>,
    pub width: u32,
    pub height: u32,
}

impl<T: KeyHandler> Window<T> {
    pub fn new(width: u32, height: u32, title: &str) -> Window<T> {
        let mut glfw = glfw::init(FAIL_ON_ERRORS).unwrap();
        let (mut window, events) = glfw
            .create_window(width, height, title, WindowMode::Windowed)
            .expect("Failed to create GLFW window.");


        Window {
            glfw,
            glfw_win: window,
            events,
            width,
            height,
            key_handler: None,
        }
    }

    fn make_current(&mut self) {
        self.glfw_win.make_current();
    }
    pub fn update_screen(&mut self) {
        self.glfw.poll_events();
        self.glfw_win.swap_buffers();
    }

    pub fn set_key_handler(&mut self, key_handler: T) {
        self.key_handler = Some(key_handler);
    }
    pub fn init_gl(&mut self) {
        gl::load_with(|s| self.glfw_win.get_proc_address(s) as *const _);
    }

    pub fn should_close(&mut self) -> bool {
        self.glfw_win.should_close()
    }
}

use crate::core::application::KartApple;
use glfw::{Action, Key, Modifiers, Scancode, WindowEvent};

pub trait AppScaffold {
    unsafe fn on_init(&mut self, app: &mut KartApple);
    unsafe fn on_loop(&mut self, app: &mut KartApple);
    unsafe fn on_resize(&mut self, width: i32, height: i32, app: &mut KartApple);
    unsafe fn on_clean(&mut self, app: &mut KartApple);
}

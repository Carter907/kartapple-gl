pub trait MouseHandler {
    fn on_mouse_down();
    fn on_mouse_up();
    fn on_mouse_released();
    fn on_mouse_click();
}

pub struct DefaultMouseHandler;

impl MouseHandler for DefaultMouseHandler {
    fn on_mouse_down() {

    }

    fn on_mouse_up() {
    }

    fn on_mouse_released() {
    }

    fn on_mouse_click() {
    }
}
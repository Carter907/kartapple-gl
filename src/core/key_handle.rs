/// struct for handling key logic.
pub trait KeyHandler {
    fn on_key_pressed(&mut self);
    fn on_key_down(&mut self);
    fn on_key_released(&mut self);
    fn on_key_up(&mut self);
}
pub struct DefaultKeyHandler;

impl KeyHandler for DefaultKeyHandler {
    fn on_key_pressed(&mut self) {}
    fn on_key_down(&mut self) {}
    fn on_key_released(&mut self) {}
    fn on_key_up(&mut self) {}
}


# kart-graphics-engine
rust graphics engine
```rust
fn main() {
    unsafe {
        // create a new kart application to store the state of your app
        let mut app = kart_application::Kartappl::new(700, 500, "hello world");
        app.init();
        let mut program = GLuint::from(1u32);
        gl::Enable(gl::DEPTH_TEST);

        let frag_code = String::from_utf8_lossy(include_bytes!("../shaders/frag.glsl")).to_string();
        let vert_code = String::from_utf8_lossy(include_bytes!("../shaders/vert.glsl")).to_string();

        program = ProgramUtils::create_program(&vert_code, &frag_code);
        app.set_program(program);
    } 
}
```

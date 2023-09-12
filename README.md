# KartApple-GL


<img src="github/assets/KartApple.png" alt="KartApple icon" style="clip-path: circle();">

#### Rust opengl library for creating whatever you want in opengl!

### Goals 
- Lightweight 
- Flexible for general purpose graphics
- Easy to use for beginners

```rust
fn main() {
    unsafe {
        let mut app = Kartappl::new(700, 500, "KartApple-GL");
        app.init();
        gl::Enable(gl::DEPTH_TEST);

        let mut program = GLuint::from(1u32);
        let vert_code = include_str!("../shaders/vert.glsl").to_string();
        let frag_code = include_str!("../shaders/frag.glsl").to_string();

        program = ProgramUtils::create_program(&vert_code, &frag_code);

        app.set_program(program);
        
        //...
    } 
}
```
![Picture_of_3D_cube.png](github%2Fassets%2FPicture_of_3D_cube.png)
## Requirements

- must have cmake installed for glfw
```bash
sudo apt install cmake
```
```bash
choco install cmake
```

## Quick Start




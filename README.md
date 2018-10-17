# glfw-wrapper

A safe and thin wrapper around the [GLFW] window and context creation library.

## Using

```toml
[dependencies.glfw-wrapper]
git = "https://github.com/MinusKelvin/glfw-wrapper.git"
```

Normally, `glfw-wrapper` will try to compile the GLFW library. To disable this, add
`default-features = false`, but you will have to provide it yourself.

## Example

```rust
let glfw = glfw_wrapper::init(&[]).unwrap();

let window = glfw.create_window(
    &WindowHints::default(),
    800, 600,
    "Basic Example",
    None, None
).unwrap();

unsafe {
    window.make_context_current().unwrap();
    gl::load_with(|s| glfw.get_proc_address(s).unwrap() as *const _);
    gl::ClearColor(0.0, 0.25, 0.5, 1.0);
}

while !window.should_close() {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
    window.swap_buffers().unwrap();
    glfw.poll_events().unwrap();
}
```

For more examples see the examples directory.

[GLFW]: http://www.glfw.org
# Graphiclity: A Minimal Graphics Library

Graphiclity is a minimal graphics library that makes drawing pixels to a window simple.

If you've ever tried to draw pixels to a window in Rust for example, for a CHIP-8 emulator you know the pain. Using `iced` or `egui` often feels like overkill, while `wgpu` + `winit` feels far too low-level.

**_Graphiclity_ exists to bridge that gap.**

It provides a small, immediate-mode drawing API on top of a window and render loop, without forcing a UI framework or architectural style.

---

## Graphiclity is not!

- A UI framework
- A widget or layout system
- A retained scene graph
- A game engine
- An alternative for `iced` or `egui`
- A replacement for `wgpu`

If you need those things, Graphiclity is probably not the right tool and that’s okay, it doesn't have to be.

---
## Features
- ✅ Simple pixel and shape drawing (rectangles, pixels)
- ✅ Built-in 8×8 bitmap font for text rendering
- ✅ Automatic DPI scaling
- ✅ Fixed logical resolution (perfect for pixel-art games and emulators)
- ✅ Resizable windows with aspect ratio preservation
- ✅ Zero-setup: just call `run()` and start drawing

## Example

```rust
use graphiclity::{run, Color};

fn main() {
    run(|g| {
        g.clear(Color::BLACK);
        g.rect(10, 10, 64, 32, Color::WHITE);
    });
}
```


---

## When to use Graphiclity?

- Developing a Emulator(Especially a Retro one) = Perfect ✅
- Developing Software in Rust that needs basic Graphical Output = Perfect ✅
- Developing a UI Framework taking Graphiclity as a foundation = Plausible ⚠️(Extensibility API is not yet implemented)
- Developing a Full fledged Game or a Desktop Application = Nope ❌ You'll struggle and will need to implement most of the things by yourself

---

## How it works

Internally, Graphiclity handles:

- Window creation and events
- A simple render loop
- Pixel drawing
- DPI Controls

You control **what** is drawn.
Graphiclity handles **how** it appears on screen.

---

## Philosophy

Graphiclity is designed to be the **first mile** of graphics in Rust.

It favors:

- Explicit code over magic
- Simplicity over complexity
- Approachability over completeness

Drawing a rectangle should not require hundreds of lines of setup.

---

## Limitations
- **No low-level access**: Graphiclity intentionally hides renderer internals to keep things simple
- **Letterboxing**: The current backend (Pixels 0.14) adds black bars when window aspect ratio doesn't match logical resolution
  - **Solution**: Use matching aspect ratios (e.g., logical 320×240 with window 640×480, both 4:3)
- **No audio**: Graphiclity is graphics-only
- **Single window**: One window per application

## Status

Graphiclity is currently under heavy development.
APIs may change as the library is refined through real-world use.

## Final Notes
Any Kind of Contributions are welcome! From Extending to Changing out the Core check out the [CONTRIBUTING.md](./CONTRIBUTING.md)


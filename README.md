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

### Basic Drawing
```rust
use graphiclity::{run, Color};

fn main() {
    run(|g| {
        g.clear(Color::WHITE);
        g.rect(50, 50, 100, 100, Color::BLACK);
    });
}
```
![drawing](./assets/drawing_demo.png)
### And A bouncing Rect
```rust
use graphiclity::Color;
fn main() {
    let mut x: i32 = 50;
    let mut y: i32 = 50;
    let mut vx: i32 = 2;
    let mut vy: i32 = 3;
    let rect_w: i32 = 20;
    let rect_h: i32 = 20;
    
    graphiclity::run(conf, move |g| {
        let (width, height) = g.logical_size();
        
        g.clear(Color::WHITE);
        
        // Draw title
        g.text(10, 10, "Bouncing Rectangle Demo", Color::CYAN);
        
        // Draw coordinates
        g.text(10, 220, &format!("X: {} Y: {}", x, y), Color::BLACK);
        
        x += vx;
        y += vy;
        
        if x <= 0 || x + rect_w >= width as i32 {
            vx = -vx;
        }
        if y <= 0 || y + rect_h >= height as i32 {
            vy = -vy;
        }
        
        g.rect(x, y, rect_w, rect_h, Color::rgb(128, 23, 255));
    });
}
```
![demo](./assets/graphliclity_demo.gif)

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


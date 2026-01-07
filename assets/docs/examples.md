### Drawing
```rust
use graphiclity::{Color };

fn main() {
    /// Run is the main entry point of Graphiclity.
    /// It starts the Event loop and Initializes the Window.
    graphiclity::run(|ctx| {
        let g = ctx.graphics(); // We are only gonna draw so lets get the graphics
        g.clear(Color::WHITE) // Lets Clear our canvas at the start of the frame
        g.pixel((5,5), Color::RED); // Draw a single pixel in cords x:5 y:5
        // Draw a rectangle: (x, y), (width, height)
        g.rect((20, 20), (50, 50), Color::BLUE);
        // Draw some text too
        g.text((20, 100), "drawingg", Color::WHITE);
    });
}

```
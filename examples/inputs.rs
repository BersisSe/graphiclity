use graphiclity::{run, Color};
use winit::event::MouseButton;
use winit::keyboard::KeyCode;

fn main() {
    let mut x = 40;
    let mut y = 40;
    let speed = 2;
    let mut color = Color::BLACK;
    
    run(|ctx| {
        let (g, input) = ctx.split();

        g.clear(Color::WHITE);

        // --- Keyboard movement ---
        if input.key_down(KeyCode::ArrowLeft) {
            x -= speed;
        }
        if input.key_down(KeyCode::ArrowRight) {
            x += speed;
        }
        if input.key_down(KeyCode::ArrowUp) {
            y -= speed;
        }
        if input.key_down(KeyCode::ArrowDown) {
            y += speed;
        }

        // --- Mouse interaction ---
        if input.mouse_pressed(MouseButton::Left) {
            color = Color::rgb(255, 0, 0);
        }
        if input.mouse_released(MouseButton::Left) {
            color = Color::BLACK;
        }

        // --- Window feedback ---
        if let Some((w, h)) = input.window_resized() {
            g.text(
                5,
                5,
                &format!("Window: {}x{}", w, h),
                Color::BLUE,
            );
        }

        // --- Draw ---
        g.rect(x, y, 20, 20, color);
    });
}

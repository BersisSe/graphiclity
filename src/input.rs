use winit::event::MouseButton;
use winit::keyboard::KeyCode;
use winit_input_helper::WinitInputHelper;

use crate::Graphics;

pub struct Input {
    pub(crate) helper: WinitInputHelper,
    mouse_logical: Option<(f32,f32)>,
}

impl Input {
    pub(crate) fn new() -> Self {
        Self {
            helper: WinitInputHelper::new(),
            mouse_logical: None,
        }
    }

    // ? Keyboard

    /// Returns true while the key is held down
    pub fn key_down(&self, key: KeyCode) -> bool {
        self.helper.key_held(key)
    }

    /// Returns true only on the frame the key was pressed
    pub fn key_pressed(&self, key: KeyCode) -> bool {
        self.helper.key_pressed(key)
    }

    /// Returns true only on the frame the key was released
    pub fn key_released(&self, key: KeyCode) -> bool {
        self.helper.key_released(key)
    }

    // ? Mouse
    pub(crate) fn update_mouse_mapping(&mut self, gfx: &Graphics) {
        self.mouse_logical = self.helper.cursor().map(|(mx, my)| {
            let (lw, lh) = gfx.logical_size();
            let (ww, wh) = gfx.window_size();

            let x = mx * lw as f32 / ww as f32;
            let y = my * lh as f32 / wh as f32;
            return (x , y);
        });
    }
    /// Returns true if the MouseButton is down in the current frame.
    pub fn mouse_down(&self, button: MouseButton) -> bool {
        self.helper.mouse_held(button)
    }
    /// Returns true if the MouseButton is pressed.
    pub fn mouse_pressed(&self, button: MouseButton) -> bool {
        self.helper.mouse_pressed(button)
    }
    /// Returns true if the MouseButton is released in the current frame.
    pub fn mouse_released(&self, button: MouseButton) -> bool {
        self.helper.mouse_released(button)
    }
    /// Returns the current Mouse position as `f32 Tuple` containg X&Y cordinates. <br>
    /// While window is unfocused it will return `None`
    /// _Note : Mouse Positions need to be precise thats why we didn't use Vec2 here!_
    pub fn mouse_pos(&self) -> Option<(f32, f32)> {
        self.mouse_logical
    }

    // ? Window Related
    /// Returns the window Size after a resize event.
    pub fn window_resized(&self) -> Option<(u32, u32)> {
        self.helper
            .window_resized()
            .map(|size| (size.width, size.height))
    }
    
    pub fn window_close_requested(&self) -> bool {
        self.helper.close_requested()
    }
}

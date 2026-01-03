use winit::event::MouseButton;
use winit::keyboard::KeyCode;
use winit_input_helper::WinitInputHelper;

pub struct Input {
    pub(crate) helper: WinitInputHelper,
}

impl Input {
    pub(crate) fn new() -> Self {
        Self {
            helper: WinitInputHelper::new(),
        }
    }

    /* =========================
     * Keyboard
     * ========================= */

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

    pub fn mouse_down(&self, button: MouseButton) -> bool {
        self.helper.mouse_held(button)
    }

    pub fn mouse_pressed(&self, button: MouseButton) -> bool {
        self.helper.mouse_pressed(button)
    }

    pub fn mouse_released(&self, button: MouseButton) -> bool {
        self.helper.mouse_released(button)
    }

    pub fn window_resized(&self) -> Option<(u32, u32)> {
        self.helper
            .window_resized()
            .map(|size| (size.width, size.height))
    }

    pub fn window_close_requested(&self) -> bool {
        self.helper.close_requested()
    }
    
}

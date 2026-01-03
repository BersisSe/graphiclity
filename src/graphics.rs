use winit::dpi::{LogicalSize, PhysicalSize};

use crate::Color;

pub enum DrawCommand {
    Clear(Color),
    Pixel {
        x: u32,
        y: u32,
        color: Color,
    },
    Line{
        x0: i32,
        y0: i32,
        x1: i32,
        y1: i32,
        color: Color,
    },
    Rect {
        y: u32,
        x: u32,
        w: u32,
        h: u32,
        color: Color,
    },
    Triangle{
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
        color: Color,
    },
    Text {
        x: i32,
        y: i32,
        text: String,
        color: Color,
    },
}

pub struct Graphics {
    commands: Vec<DrawCommand>,
    logic_width: u32,
    logic_height: u32,

    window_width: u32,
    window_height: u32,
}

impl Graphics {
    /// Create a new Graphics context
    pub(crate) fn new(logic_size: LogicalSize<u32>, phy_size: PhysicalSize<u32>) -> Self {
        Graphics {
            commands: Vec::with_capacity(128),
            logic_height: logic_size.height,
            logic_width: logic_size.width,
            window_height: phy_size.height,
            window_width: phy_size.width,
        }
    }
    /// Clear commands at the start of each frame
    /// This should be called internally by the runtime at the start of a redraw
    pub(crate) fn begin_frame(&mut self) {
        self.commands.clear();
    }
    /// Get the list of draw commands for this frame
    pub(crate) fn commands(&self) -> &[DrawCommand] {
        &self.commands
    }
    /// Get the logical size of the window
    pub fn logical_size(&self) -> (u32, u32) {
        (self.logic_width, self.logic_height)
    }
    /// Get the physical window size
    pub fn window_size(&self) -> (u32, u32) {
        (self.window_width, self.window_height)
    }
    /// Clear the screen with a color  
    /// You should call this at the start of each frame to clear the previous frame's drawings
    pub fn clear(&mut self, color: Color) {
        self.commands.push(DrawCommand::Clear(color));
    }
    /// Draw a single pixel at (x, y) with the specified color
    pub fn pixel(&mut self, x: i32, y: i32, color: Color) {
       
        if x < 0 || y < 0 {
            return;
        }

        self.commands.push(DrawCommand::Pixel {
            x: x as u32,
            y: y as u32,
            color,
        });
    }
    /// Draw a line from (x0, y0) to (x1, y1) with the specified color
    pub fn line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
        // Guards
        if x0 < 0 && x1 < 0 {
            return;
        }
        if y0 < 0 && y1 < 0 {
            return;
        }

        self.commands.push(DrawCommand::Line {
            x0,
            y0,
            x1,
            y1, 
            color,
        });
    }
    /// Draw a rectangle at (x, y) with width w and height h with the specified color
    pub fn rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: Color) {
        // Handle negative x/y by shifting the start and shrinking the width/height
        let mut rx = x;
        let mut ry = y;
        let mut rw = w;
        let mut rh = h;

        if rx < 0 {
            rw += rx; // Reduce width by the amount it's off-screen
            rx = 0;
        }
        if ry < 0 {
            rh += ry; // Reduce height by the amount it's off-screen
            ry = 0;
        }

        // If there is still something to draw
        if rw > 0 && rh > 0 {
            self.commands.push(DrawCommand::Rect {
                x: rx as u32,
                y: ry as u32,
                w: rw as u32,
                h: rh as u32,
                color,
            });
        }
    }
    /// Draw a triangle with vertices at (x1, y1), (x2, y2), (x3, y3) with the specified color
    pub fn triangle(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, color: Color) {
        self.commands.push(DrawCommand::Triangle {
            x1,
            y1,
            x2,
            y2,
            x3,
            y3,
            color,
        });
        
    }
    /// Draw text at (x, y) with the specified color using the internal bitmap font
    pub fn text<T: Into<String>>(&mut self, x: i32, y: i32, text: T, color: Color) {
        self.commands.push(DrawCommand::Text {
            x,
            y,
            text: text.into(),
            color,
        });
    }
}

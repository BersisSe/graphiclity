use crate::{Graphics, Input};

pub struct WindowContext {
    pub(crate) gfx: Graphics,
    pub(crate) inputs: Input,
}

impl WindowContext {
    pub(crate) fn new(gfx : Graphics, inputs: Input) -> Self {
        WindowContext {
            gfx,
            inputs,
        }
    }
    pub fn graphics(&mut self) -> &mut Graphics{
        &mut self.gfx
    }
    pub fn input(&self) -> &Input{
        &self.inputs
    }
    /// Splits the Graphics and Input from the Context
    pub fn split(&mut self) -> (&mut Graphics, &Input){
        return (&mut self.gfx, &self.inputs);
    }
    
}

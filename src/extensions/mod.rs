use crate::{FrameContext};


pub trait Extension {
    /// Setup your extension
    fn on_init(&mut self){}
    /// Called Before Drawing
    fn pre_draw(&mut self, _ctx: &mut FrameContext) {}
    // Called After Drawing
    fn post_draw(&mut self , _ctx: &mut FrameContext){}
 
}


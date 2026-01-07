
/// Vec2 Represents a 2 Dimentional point. <br>
/// (i32,i32) and (f32,f32) Tuples could be converted to `Vec2` by using `into`.  <br>
/// _Note : When using `into` with floating point numbers it gets casted into i32(using .floor) and loses precision_
#[derive(Clone, Copy, Debug)]
pub struct Vec2{
    pub x: i32,
    pub y: i32
}

impl Vec2 {
    /// Contruct a new Vec2 from X & Y cordinates
    pub fn new(x: i32, y: i32) -> Vec2{
        Vec2 { x, y }
    }
    pub fn as_u32_tuple(&self) -> (u32, u32) {
        (self.x.max(0) as u32, self.y.max(0) as u32)
    }
}

impl Into<Vec2> for (i32,i32) {
    fn into(self) -> Vec2 {
        Vec2 { x: self.0, y: self.1 }
    }
}

impl Into<Vec2> for (f32,f32) {
    fn into(self) -> Vec2 {
        Vec2 { 
            x: self.0.floor() as i32,

            y: self.1.floor() as i32, 
        }
    }
}

impl std::ops::Add for Vec2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vec2 { x: self.x + other.x, y: self.y + other.y }
    }
}
impl std::ops::Sub for Vec2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Vec2 { x: self.x - other.x, y: self.y - other.y }
    }
}

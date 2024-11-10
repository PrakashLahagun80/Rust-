pub struct Rectangle {
   pub width: u32,
   pub height: u32
}

impl Rectangle {
    pub fn area(&self)->u32 {
        return self.width * self.height;
    }
}
pub struct Rect {
    pub height: f32,
    pub width: f32,
}

impl Rect {

    // dynamic function
    pub fn area(&self) -> f32 {
        return self.width * self.height;
    }

    // dynamic function
    pub fn perimeter(&self) -> f32 {
        return 2.0*(self.width + self.height);
    }

    // static function
    pub fn print_something(a: u32){ 
        println!("{}", a);
    }
}
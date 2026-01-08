use std::f32::consts::PI;


// Enums with value
pub enum Shape{
    Square(f32),
    Circle(f32),
    Rectangle(f32, f32)
}

impl Shape {
    pub fn calculate_area(&self) -> f32{
        return match self{
            Shape::Circle(radius) => PI * radius * radius,
            Shape::Square(side) => side * side,
            Shape::Rectangle(length, breadth) => length * breadth,
        };
    }
}
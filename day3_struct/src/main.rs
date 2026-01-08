use rect::Rect;
use shapes::Shape;
use direction::Direction;

pub mod rect;
pub mod shapes;
pub mod direction;

// #[derive(PartialEq)]


// Normal/Simple Enum





fn main() {
    let r = Rect{
        width:  10.00,
        height: 11.00
    };

    println!("Width: {}, Height: {}", r.width, r.height);

    println!("Area: {}" , r.area());
    println!("Perimeter: {}" , r.perimeter());
    Rect::print_something(15);

    let dir = Direction::West;
    steer(dir);


    let cir = Shape::Circle(10.00);
    let sq = Shape::Square(10.00);
    let rec = Shape::Rectangle(10.00, 11.00);

    println!("Circle: {}, Square: {}, Rectangle: {}", cir.calculate_area(), sq.calculate_area(), rec.calculate_area());

}

fn steer(direc: Direction) {
    // if direc == Direction::North  {
    //     println!("Moves to North");
    // }
    // if direc == Direction::South  {
    //     println!("Moves to South");
    // }
    // if direc == Direction::East  {
    //     println!("Moves to East");
    // }
    // if direc == Direction::West  {
    //     println!("Moves to West");
    // }

    // Pattern matching
    match direc{
        Direction::North => println!("Moves to North"),
        Direction::South => println!("Moves to South"),
        _ => println!("Horizontal"),
    }
}




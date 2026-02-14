


trait Shape {
    fn area(&self) -> u32;
}

struct Rect {
    width: u32,
    height: u32
}

impl Shape for Rect {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
}

struct Square {
    side: u32
}

impl Shape for Square {
    fn area(&self) -> u32 {
        return self.side * self.side;
    }
}

fn main() {
    println!("Hello, world!");
    let r = Rect{
        width: 10,
        height: 12
    };

    let sq = Square {
        side: 25
    };

    let s1 = get_area(r);
    let s2 = get_area(sq);
    println!("{}", s1);
    println!("{}", s2);
}



fn get_area(s: impl Shape) -> u32 {
    return s.area();
}
// Traits and Generics

fn main() {
    println!("Hello, world!");
    let s1 = sum(1.1, 2.0);
    println!("{}",s1);
    let s2 = sum(3, 2);
    print!("{}", s2);
}


fn sum<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    return a + b;
}
fn main() {
    println!("Hello, world!");
    let ans = sum(1,2);
    println!("{}", ans);
}

fn sum(a: u32, b: u32) -> u32 {
    return a+b; 
}
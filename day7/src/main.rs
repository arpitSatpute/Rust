// macro and macro_rules


macro_rules! say_hello {
    () => {
        println!("Hello World");
    }
}


fn main() {
    say_hello!();
}

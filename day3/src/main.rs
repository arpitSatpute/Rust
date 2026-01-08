// Instead of Borrowing the value from other function, here we are passing the address of the string/value.
// So that, we can still use `str` after passing the address of string to get_length function.
// This is called Borrowing

fn main() {
    let str = String::from("Yuvansh");
    let len = get_length(&str);
    println!("{} {}", str, len);
    immutable_reference();
    mutable_reference();
}

fn get_length(str : &String) -> usize {
    let len = str.len();
    return len;
}


// Borrowing rules: 
    // 1. there can be only one mutable refrence.
    // 2. there can be multiple immutable refrences.


fn immutable_reference() {
    let str = String::from("Arpit");  // Owner variable
    let s2 = &str;  //immutable reference
    let s3 = &str;  //immutable reference
    let s4 = &str;  //immutable reference

    println!("{}, {}, {}, {}", str, s2, s3, s4);
}


fn mutable_reference() {
    let mut s1 = String::from("Astar");
    let s5 = &mut s1;   // mutable reference
    s5.push_str(" S");
    println!("{}", s5);  // mutable reference scope ends here

    // here the code works as the scope of `s5` i.e mutable variable is ended above. it is never used after the above.
    let s6 = &s1;  //immutable reference
    let s7 = &s1;  //immutable reference
    println!("{}, {}", s6, s7);
}   
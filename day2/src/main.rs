fn main() {
    mutable_string(); // Mutable String. Can be changed if created with `mut`.
    immutable_string(); // Immutable String. Once created cannot be changed.

    // Ownership transferback
    let str = String::from("Yuvansh");
    let (str2, len) = get_length(str);
    println!("{} {}", str2, len);

    // Ownership Transfer
    
    // let new_str = str;  // loose ownership of "Yuvansh". Now transfer to new_str.
    let new_str = str2.clone(); //void ownership transfer. maintain separate ownership for same value but with different memory allocation.
    println!("{}", new_str);
    println!("{}", str2);
}

fn get_length(str : String) -> (String, usize) {
    let len = str.len();
    return (str, len);
}

fn mutable_string() {
    let mut name: String = String::from("Arpit");
    name.push_str(" Satpute");
    println!("{}", name);
}

fn immutable_string() {
    let name: String = String::from("Astar");
    println!("{}", name);
}

// Ownership rules: 1. Each value has an owner
                //  2. There can be one owner at a time. 
                //  3. when owner goes out of scope value drops.

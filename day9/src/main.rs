
use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]

struct User {
    username: String,
    password: String
}

fn main() {

    let u = User {
        username: String::from("Arpit"),
        password: String::from("123123123")
    };

    let bytes = borsh::to_vec(&u).unwrap();
    println!("{:?}", bytes);

    let u2: User = borsh::from_slice(&bytes).unwrap();
    println!("{:?}", u2);
    
}

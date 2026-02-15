// SERDE

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]

struct User {
    username: String,
    password: String
}

fn main() {

    let u = User {
        username: String::from("Arpit"),
        password: String::from("123123123")
    };

    let serialized_string = serde_json::to_string(&u).unwrap();
    println!("{}", serialized_string);

    let u2 = r#"{"username":"Arpit","password":"123123123"}"#;
    let s1: Result<User, serde_json::Error> = serde_json::from_str(u2);
    println!("{:?}", s1.unwrap());

}

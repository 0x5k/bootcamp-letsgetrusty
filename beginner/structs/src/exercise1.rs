struct User {
    name: String,
    age: u8,
    active: bool,
}

fn main() {
    let user = User {
        name: String::from("user1"),
        age: 17u8,
        active: true,
    };
    println!("User's name: {}", user.name);
    println!("User's age: {}", user.age);
    println!("User's status: {}", user.active);
}

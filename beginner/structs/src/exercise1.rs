struct User {
    name: String,
    age: u8,
    active: bool,
}

impl User {
    fn new(name: String, age: u8, active: bool) -> Self {
        Self { name, age, active }
    }
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

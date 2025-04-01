struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Helper function to create a new User with default values
fn build_user(email: String, username: String, active: bool, sign_in_count: u64) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 111,
    }
}

fn main() {
    // Creates first user using the build_user function
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
        true,
        1,
    );

    // Creates second user, copying some fields from user1
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        //active: user1.active,
        //sign_in_count: user1.sign_in_count,
        ..user1 // This means "copy all other fields from user1"
    };

    println!("user1: {}, {}", user1.email, user1.username);
    println!("user2: {}, {}", user2.email, user2.username);
    println!(
        "user1 active: {}, user1 sign_in_count: {}",
        user1.active, user1.sign_in_count
    );
    println!(
        "user2 active: {}, user2 sign_in_count: {}",
        user2.active, user2.sign_in_count
    );
}

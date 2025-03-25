fn main() {
    //creation

    let x: i32 = 5;
    println!("value of x is: {x}");

    // constant. you can do operations when you assign it

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS} seconds in a constant");

    // static is like constant, but it can be mutable with mut keyword.
    // be careful with mut static, it can be changed.
    // also it's scope is global.

    // Avoid when possible
    static mut UNSAFE_COUNTER: u32 = 0;

    // Better alternative
    use std::sync::atomic::{AtomicU32, Ordering};
    static SAFE_COUNTER: AtomicU32 = AtomicU32::new(0);

    // Constant (preferred when possible)
    const MAX_HEALTH: u32 = 100;
    println!("max HP is: {MAX_HEALTH}");

    // Static (when you need a fixed memory address)
    static GAME_NAME: &str = "Super Rust Bros";
    println!("game name is: {GAME_NAME}");

    // Immutable by default
    let x = 5;
    // x = 6;  // This would cause a compilation error

    // Mutable variables
    let mut y = 5;
    y = 6; // This is fine

    // Shadowing (creating a new variable)
    let value = 5;
    let value = value + 1; // Creates new immutable variable
    let value = value * 2; // Creates another new variable
    println!("Value: {}", value); // 12

    // vs Mutation (changing existing variable)
    let mut count = 5;
    count = count + 1; // Modifies existing variable
    count = count * 2; // Modifies same variable
    println!("Count: {}", count); // 12

    // scope, the variable is only available in the block it is defined in.
    let hello = "Hello, world!";

    {
        let hello = "inner Hello world";
        println!("{hello}");
    }
    println!("{hello}");
}

fn main() {
    //creation

    let x: i32 = 5;
    println!("value of x is: {x}");

    // constant, you can do operations when you assign it

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS} seconds");

    // mutability, you can reassign the value of a variable, but you can't do it with const or not mut let.
    // with mutability you works with the same variables.

    let mut y: i32 = 10;
    println!("value of y is: {y}");

    y = 20;
    println!("value of y is: {y}");

    // shadowing, you can reassign the same variable name to a new value, and also change the type it has.
    // with shadowing you have two different variables, the first one is immutable and the second one is mutable.

    let s = 17;
    let s = s + 1;

    println!("this is s: {s}");
    // scope, the variable is only available in the block it is defined in.
    let hello = "Hello, world!";

    {
        let hello = "inner Hello world";
        println!("{hello}");
    }
    println!("{hello}");
}

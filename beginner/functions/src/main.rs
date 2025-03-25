fn main() {
    let z: i32 = side_function(1337); // z gets the value returned by side_function
    println!("side_function returned {}", z); // will print 10, not 1337
}

fn side_function(x: i32) -> i32 {
    // takes x as input, returns an i32
    println!("side_function called with {}", x); // this is an statement, ends with a semicolon
    let y: i32 = 10; // creates a new variable y
    y // returns y (which is 10), semicolon is omitted, this is an expression
}

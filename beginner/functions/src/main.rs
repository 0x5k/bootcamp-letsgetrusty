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
//  Implicit return

// fn example1(x: i32) -> i32 {
//     let y = 10;
//     y  // returns 10
// }

// Explicit return

// fn example2(x: i32) -> i32 {
//     return 10;  // same result, more explicit
// }

// Early return
// fn example3(x: i32) -> i32 {
//     if x < 0 {
//         return 0;  // early return if x is negative
//     }
//     x  // return x if it's not negative

// No return value (returns unit type '()')
// fn print_number(x: i32) {
//     println!("{}", x);

/* Functions can take multiple parameters
The return type is specified after ->
The last expression (without semicolon) is returned
You can use return keyword for explicit returns
If no return value is specified, the function returns unit type () */

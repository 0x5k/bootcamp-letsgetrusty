fn main() {
    let s1 = String::from("Rust"); // heap allocated string
    let s2 = s1.clone();
    print_string(s1.clone());
    let s3 = generate_string();
    let s4 = add_to_string(s2);

    println!("s1 is: {s1}");
    println!("s3 is: {s3}");
    println!("s4 is: {s4}");

    let x = 10;
    let y = x;
    print_integer(x);
    println!("x is: {x}");
    // x is still valid because it is a primitive type, copied on stack
    // cloning a primitive type is cheap, because the value is copied on stack and they cloned by default
}

fn print_string(p1: String) {
    println!("{p1}");
} // p1 is dropped

fn print_integer(i: i32) {
    println!("i is: {i}");
}

fn generate_string() -> String {
    String::from("Ferris")
}

fn add_to_string(mut p1: String) -> String {
    p1.push_str(" is awesome!");
    p1
}

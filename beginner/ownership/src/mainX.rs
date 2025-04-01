fn main() {
    let s1 = String::from("Rust ");
    let s2 = s1.clone();
    //print_str(s2);
    let s3 = add_to_str(s1);
    print_str(s3);
}

fn add_to_str(mut p1: String) -> String {
    p1.push_str("is awesome!");
    // (*p1).push_str(" is awesome!") - automatically dereferences the pointer
    p1
}

fn print_str(p1: String) {
    println!("{p1}");
}

/*
fn main() {
    // Example 1: Clone
    // let s1 = String::from("Rust ");
    // let s2 = s1.clone();
    // println!("Clone - s1: {s1}, s2: {s2}");

    // Example 2: Reference
    // let s1 = String::from("Rust ");
    // let s2 = &s1;
    // println!("Reference - s1: {s1}, s2: {s2}");

    // Example 3: Mutable Reference
    // let mut s1 = String::from("Rust ");
    // {
    //     let s2 = &mut s1;
    //     s2.push_str("is great!");
    //     println!("Mut Reference - s2: {s2}");
    // } // s2 goes out of scope here
    // println!("Mut Reference - s1: {s1}"); // Now we can use s1 again

    // Example 4: Move
    // let s1 = String::from("Rust ");
    // let s2 = s1;
    // println!("Move - s2: {s2}");
    // println!("s1: {s1}"); // This would fail
} */

// Ownership Rules:
// Each value has exactly one owner
// When owner goes out of scope, value is dropped
// When you move a value, the original variable becomes invalid
// Borrowing Rules:
// You can have either ONE mutable reference OR ANY NUMBER of immutable references
// References must always be valid (no dangling references)
// References can't outlive the original data
// When to Use Each:
// Use clone() when you need a completely independent copy
// Use references (&) when you just need to read the data
// Use mutable references (&mut) when you need to modify the borrowed data
// Use moves when you want to transfer ownership
// Performance Considerations:
// clone() is most expensive (copies all data)
// References are cheapest (just a pointer)
// Moves are free (just transfers ownership)

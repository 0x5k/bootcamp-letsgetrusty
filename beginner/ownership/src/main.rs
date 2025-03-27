/// Ownership in Rust is a system that manages memory allocation and deallocation for variables.
/// It ensures that each value is owned by exactly one variable.
/// When a variable goes out of scope, the value is dropped and the memory is freed.
/// Also, when a variable is moved, the original variable is no longer valid.
///
/// rustc enforces ownership rules to prevent memory errors.
///
/// integers, booleans, floats, strings, arrays, and tuples are copied on stack.
/// strings, vectors, and hashmaps are allocated on the heap.
/// ```
/// let original = String::from("Hello, Rust!"); // heap allocated string
/// let copied = original.clone();
///
/// println!("Original: {original}");
/// println!("Copied version: {copied}");
/// ```

fn main() {
    let str1 = String::from("some string"); // heap allocated string
    let str2 = str1.clone();
    // let str2 = str1;
    // this would move the value of str1 to str2, and str1 would be invalid

    println!("some str1: {str1}");
    println!("some str2 cloned: {str2}");

    {
        let str3 = String::from("more strings");
        let str4 = str3;
        //println!("this would not print {str3}") //value borrowed here after move
        // but it will print str4
        println!("this will print {str4}");
    }
}

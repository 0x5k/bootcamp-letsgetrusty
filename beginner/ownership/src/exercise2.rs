// Explanation of the code:

// The `main` function is the entry point of the program.
fn main() {
    // 1. Create a mutable variable `str1`.
    //    `get_new_string()` creates a String "I will master rust 🦀 🦀" on the heap.
    //    Ownership of this String is *moved* from `get_new_string` to `str1`.
    //    `str1` now owns the String data.
    let mut str1 = get_new_string();

    // 2. Print the value of `str1`. This works because `str1` owns the String.
    //    `println!` macro borrows `str1` immutably to read its value.
    println!("Printing through str1: {}", str1);

    // 3. Create a mutable variable `str2` and assign `str1` to it.
    //    Because `String` is a type that owns data on the heap and does not implement
    //    the `Copy` trait, this assignment is a *move*.
    //    Ownership of the String data is transferred from `str1` to `str2`.
    //    After this line, `str1` is no longer considered initialized or valid by the compiler,
    //    as it no longer owns the data. Trying to use `str1` here would cause a compile error.
    let mut str2 = str1;

    // 4. Print the value of `str2`. This works because `str2` now owns the String.
    println!("Printing through str2: {}", str2);

    // 5. Assign `str2` back to `str1`.
    //    This is another *move*. Ownership of the String data is transferred
    //    from `str2` back to `str1`.
    //    After this line, `str2` is no longer valid, and `str1` is valid again.
    str1 = str2;

    // 6. Print the value of `str1`. This works because `str1` owns the String again.
    println!("Again printing through str1: {}", str1);

    // 7. Assign a *clone* of `str1` to `str2`.
    //    `str1.clone()` performs a *deep copy* of the String data owned by `str1`.
    //    - It allocates new memory on the heap.
    //    - It copies the content ("I will master rust 🦀 🦀") from `str1`'s data into this new memory.
    //    - It returns a *new* `String` instance that owns this newly allocated data.
    //    Ownership of this *new* String is transferred to `str2`.
    //    Crucially, `str1` *retains* ownership of its original String data.
    //    After this line, both `str1` and `str2` are valid and own *separate* copies
    //    of the string data on the heap. Changes to one will not affect the other.
    str2 = str1.clone(); //this was a solution, add .clone() to str1

    // 8. Print the value of `str2`. This works because `str2` owns its cloned copy.
    println!("Again printing through str2: {}", str2);

    // 9. Print both `str1` and `str2`. This works because both variables are valid
    //    and own their respective String data after the `clone()` operation.
    println!("Printing thourgh both: {}, {}", str1, str2);
} // `str1` and `str2` go out of scope here. The memory they own is automatically deallocated.

// This function creates a new `String` on the heap.
fn get_new_string() -> String {
    let new_string = String::from("I will master rust 🦀 🦀");
    // When the function returns `new_string`, ownership of the String data
    // is *moved* out of this function to the caller (the variable that receives the result).
    new_string
}

/*
Could it be done better?

- For learning purposes: This code effectively demonstrates Rust's ownership, move semantics, and the `clone()` mechanism for types like `String` that manage heap data. The back-and-forth moves (`str2 = str1`, `str1 = str2`) highlight how ownership transfer invalidates the source variable. The `clone()` shows how to explicitly duplicate data when needed.

- In real-world code: Whether this is "better" depends entirely on the goal.
    - If you genuinely need two independent, mutable copies of the string data, then `clone()` is the correct and necessary approach.
    - If you only need to *read* the string data in multiple places without modifying it, using *borrows* (`&str1`, `&str2`) would be more efficient as it avoids heap allocations and data copying.
    - If you need to modify the string data via multiple variables, you might consider using shared ownership types like `Rc` (Reference Counting) or `Arc` (Atomic Reference Counting), possibly combined with interior mutability patterns (`RefCell` or `Mutex`), although these add complexity.

- Efficiency: `clone()` for `String` involves allocating new memory on the heap and copying the character data, which can be relatively expensive for large strings compared to moving ownership (which is just copying pointers/metadata on the stack) or borrowing (which is just passing a reference). You generally want to avoid unnecessary clones in performance-sensitive code.

In summary: The code is a good illustration of ownership concepts. Whether `clone()` is "good" or "bad" depends on whether you actually *need* a separate, owned copy of the data. If not, borrowing (`&`) is often preferred.
*/

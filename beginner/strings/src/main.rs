fn main() {
    // 1. Using `concat()` on a slice of string literals (&str).
    //    This method takes an array or slice of strings/string slices
    //    and concatenates them into a single *new* owned String.
    let s1: String = ["first", "second"].concat();
    // s1 now holds "firstsecond"

    // 2. Using the `format!` macro.
    //    This is the most idiomatic and flexible way to create formatted strings.
    //    It takes a format string (like `println!`) and arguments,
    //    returning a *new* owned String. It avoids ownership issues with `+`.
    let s2: String = format!("{} {}", "first", "second");
    // s2 now holds "first second"

    // 3. Using the `concat!` macro.
    //    This macro concatenates string *literals* at compile time.
    //    The result is a string literal (`&'static str`), not an owned String.
    //    It only works with literals known at compile time.
    let s3: &'static str = concat!("first", "second");
    // s3 now holds "firstsecond" (as a static string slice)

    // --- Concatenating with the `+` operator ---
    // The `+` operator for Strings uses the `add` method from the `Add` trait.
    // `impl Add<&str> for String`
    // This means the left operand *must* be an owned `String`.
    // The right operand *must* be a string slice (`&str`).
    // Crucially, the `add` method takes ownership of the left operand (`self`)
    // and returns a *new* owned String.

    // 4. Adding a string literal (`&str`) to an owned `String`.
    let s4 = String::from("test");
    // `s4` (String) + `"ok"` (&str)
    // Ownership of `s4` is MOVED into the `add` method.
    // A new String is returned and assigned to `s5`.
    let s5 = s4 + "ok"; // s4 is no longer valid after this line.
    // s5 now holds "testok"
    // println!("{}", s4); // This would cause a compile error (use of moved value)

    // 5. Adding a string slice (`&str`) from another `String` to an owned `String`.
    let s6 = String::from("test");
    let s7 = String::from("ok ok");
    // `s6` (String) + `&s7` (&str slice of s7)
    // Ownership of `s6` is MOVED into the `add` method.
    // `s7` is borrowed as a slice (`&s7`), so `s7` remains valid.
    // A new String is returned and assigned to `s8`.
    let s8 = s6 + &s7; // s6 is no longer valid after this line.
    // s8 now holds "testok ok"
    // println!("{}", s6); // Compile error
    println!("{}", s7); // This is okay, s7 was only borrowed
}

// Helper function (if you were using it, not present in original code)
// fn get_new_string() -> String {
//     let new_string = String::from("I will master rust 🦀 🦀");
//     new_string
// }

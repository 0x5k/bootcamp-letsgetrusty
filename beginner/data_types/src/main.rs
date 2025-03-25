fn main() {
    /* ------- scalar types ------- */

    // boolean
    let b1: bool = true;

    // unsigned integers
    let i1: u8 = 1;
    let i2: u16 = 1;
    let i3: u32 = 1;
    let i4: u64 = 1;
    let i6: u128 = 1;

    // signed integers
    let i7: i8 = 1;
    let i8: i16 = 1;
    let i9: i32 = 1;
    let i10: i64 = 1;
    let i11: i128 = 1;

    // floating point numbers
    let f1: f32 = 1.0;
    let f2: f64 = 1.0;

    // platform specific integers
    let p1: usize = 1;
    let p2: isize = 1;

    // characters, &str, and String
    let c1: char = 'c';
    let s1: &str = "hello";
    let s2: String = String::from("hello");
    let s3: String = "hello".to_string();
    let s4: &'static str = "hello";

    /* ------- compound types ------- */

    // arrays
    let a1 = [1, 2, 3, 4, 5];

    let i1 = a1[4];

    // tuples
    let t1 = (1, 2, 3);
    let t1 = (5, 5.0, "5");

    let s1 = t1.2;
    let (i1, f1, s1) = t1;

    let unit = ();

    // Type aliases (type synonyms) are a way to give an existing type a new name.
    // They are useful for making code more readable or for simplifying complex types.
    // Type aliases are declared using the type keyword. Also, type aliases are not new types, they are just aliases for existing types.
    // Type aliases are declared in the global scope.
    type Age = u8;

    let a1: Age = 57;

    // Type aliases can be used to simplify complex types.
    type Point = (i32, i32);

    let p1: Point = (1, 2);
}

fn main() {
    /* ------- scalar types ------- */

    // boolean
    let b1: bool = true;
    let bool2: bool = false;

    // unsigned integers
    let i1: u8 = 1; // Max value:  2^8 - 1 = 255

    let i2: u16 = 1; // Max value:  2^16 - 1 = 65,535

    let i3: u32 = 1; // Max value:  2^32 - 1 = 4,294,967,295

    let i4: u64 = 1; // Max value:  2^64 - 1 = 18,446,744,073,709,551,615

    let i6: u128 = 1; // Max value: 2^128 - 1 = 340,282,366,920,938,463,463,374,607,431,768,211,455

    // signed integers
    let i7: i8 = 1; // Max value 2^7 - 1 = 127  Min value:  -2^7 = -128

    let i8: i16 = 1; // Max value:  2^15 - 1 = 32,767   Min value:  -2^15 = -32,768

    let i9: i32 = 1; //Max value:  2^31 - 1 = 2,147,483,647      Min value:  -2^31 = -2,147,483,648

    let i10: i64 = 1; // Max value:  2^63 - 1 = 9,223,372,036,854,775,807 $ Min value:  -2^63 = -9,223,372,036,854,775,808 $

    let i11: i128 = 1; // Max value: 2^127 - 1 = 170,141,183,460,469,231,731,687,303,715,884,105,727  Min value: $ -2^127 = -170,141,183,460,469,231,731,687,303,715,884,105,728

    // floating point numbers
    let f1: f32 = 1.0; // Approx 3.4028235e38 MAX
    let f2: f64 = 1.0; // Approx 1.7976931348623157e308 MAX

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
    let a1 = [1, 2, 3, 4, 5]; // multi

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

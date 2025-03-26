fn main() {
    // if/else
    let a = 5;

    if a > 5 {
        println!("bigger than 5");
    } else if a > 3 {
        println!("bigger than 3");
    } else {
        println!("smaller or equal to 3");
    }

    let b = if a > 5 { 1 } else { -1 };

    // while loop
    let mut a = 0;

    while a < 5 {
        println!("a is {a}");
        a = a + 1;
    }

    // for loop
    let a = [1, 2, 3, 4, 5];

    for element in a {
        println!("{}", element);
    }
}

//buzfiz

// fn main() {
//     let mut n = 1;
//     while n <= 100 {
//         if n % 15 == 0 {
//             println!("fizzbuzz");
//         } else if n % 3 == 0 {
//             println!("fizz");
//         } else if n % 5 == 0 {
//             println!("buzz");
//         } else {
//             println!("{}", n);
//         }
//         n += 1;
//     }
// }

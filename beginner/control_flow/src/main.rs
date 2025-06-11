fn main() {
    let proceed = true;
    if proceed {
        println!("Proceeding");
    } else {
        println!("Not proceeding");
    }

    let age = 16;
    let gender = "male"; // Could be "male" or "female"

    if age < 3 {
        println!("child");
    } else if age < 17 {
        println!("teenager");
        // Nested if-else for gender-specific messages
        if gender == "male" {
            println!("Young man");
        } else if gender == "female" {
            println!("Young lady");
        }
    } else {
        println!("adult");
        // Nested if-else for gender-specific messages
        if gender == "male" {
            println!("Gentleman");
        } else if gender == "female" {
            println!("Lady");
        }
    }

    let height = 190;
    if height < 180 {
        println!("Tall");
    } else if height > 170 {
        println!("Average");
    } else {
        println!("Short");
    }
}

// fn main() {
//     'outer: for x in 1..=3 {
//         println!("x: {x}");

//         'inner: for y in 1..=3 {
//             if y == 2 {
//                 continue 'inner; // Skips the rest of the inner loop's current iteration
//             }
//             if x == 3 && y == 3 {
//                 continue 'outer; // Skips to the next iteration of the outer loop
//             }
//             println!("  y: {y}");
//         }
//     }
// }

// fn main() {
//     let correct_password = "rust123";
//     let max_attempts = 3;
//     let mut attempts_left = max_attempts;

//     'login: loop {
//         println!("\nAttempts left: {attempts_left}");

//         // Simulating password input (in real code, you'd use user input)
//         let password_attempt = if attempts_left == 3 { "wrong1" }
//                              else if attempts_left == 2 { "wrong2" }
//                              else { "rust123" };

//         if password_attempt == correct_password {
//             println!("Password correct! Welcome!");
//             break 'login;
//         }

//         attempts_left -= 1;
//         if attempts_left == 0 {
//             println!("No more attempts left. Account locked!");
//             break 'login;
//         }

//         println!("Incorrect password, please try again.");
//     }
// }

// fn main() {
//     let matrix = [
//         [1, 2, 3],
//         [4, 5, 6],
//         [7, 8, 9]
//     ];
//     let target = 5;

//     'outer: for i in 0..3 {
//         for j in 0..3 {
//             if matrix[i][j] == target {
//                 println!("Found {target} at position [{i}][{j}]");
//                 break 'outer;
//             }
//             println!("Checking position [{i}][{j}]: {}", matrix[i][j]);
//         }
//     }
// }

// fn main() {
//     'outer: for i in 1..=5 {
//         println!("Multiplication table for {i}:");

//         for j in 1..=10 {
//             if i == 3 && j == 6 {
//                 println!("Stopping at 3's table!");
//                 break 'outer; // Breaks out of both loops
//             }
//             println!("{i} x {j} = {}", i * j);
//         }
//         println!("---------------");
//     }
// }

// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remainder = 10;
//         loop {
//             println!("remainig = {remainder}");
//             if remainder == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remainder -= 1;
//         }
//         count += 1;
//     }
//     println!("end of count = {count}");
// }

// fn main() {
//     // if/else
//     let a = 5;

//     if a > 5 {
//         println!("bigger than 5");
//     } else if a > 3 {
//         println!("bigger than 3");
//     } else {
//         println!("smaller or equal to 3");
//     }

//     let b = if a > 5 { 1 } else { -1 };

//     // while loop
//     let mut a = 0;

//     while a < 5 {
//         println!("a is {a}");
//         a = a + 1;
//     }

//     // for loop
//     let a = [1, 2, 3, 4, 5];

//     for element in a {
//         println!("{}", element);
//     }
// }

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

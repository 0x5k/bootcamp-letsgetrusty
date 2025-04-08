enum Command {
    Undo,
    Redo,
    AddText(String),
    MoveCursor(i32, i32),
    Replace { from: String, to: String },
}

impl Command {
    fn serialize(&self) -> String {
        let json_string = match self {
            Command::Undo => String::from("{\"cmd\": \"undo\" }"),
            Command::Redo => String::from("{\"cmd\": \"undo\" }"),
            Command::AddText(s) => {
                format!(
                    "{{ \
                        \"cmd\": \"add_text\", \
                        \"text\": \"{s}\" \
                    }}"
                )
            }
            Command::MoveCursor(line, column) => {
                format!(
                    "{{ \
                        \"cmd\": \"move_cursor\", \
                        \"line\": {line}, \
                        \"column\": {column} \
                    }}"
                )
            }
            Command::Replace { from, to } => {
                format!(
                    "{{ \
                        \"cmd\": \"replace\", \
                        \"from\": \"{from}\", \
                        \"to\": \"{to}\" \
                    }}"
                )
            }
        };
        json_string
    }
}
fn main() {
    let cmd1 = Command::Undo;
    let cmd2 = Command::Redo;
    let cmd3 = Command::AddText(String::from("test"));
    let cmd4 = Command::MoveCursor(27, 0);
    let cmd5 = Command::Replace {
        from: String::from("test1"),
        to: String::from("test2"),
    };

    println!("{}", cmd1.serialize());
    println!("{}", cmd2.serialize());
    println!("{}", cmd3.serialize());
    println!("{}", cmd4.serialize());
    println!("{}", cmd5.serialize());

    // let age = 33;
    // match age {
    //     1 => println!("Happy 1st BD"),
    //     13..=19 => println!(" you'r teen"),
    //     x => println!("You are {x} years old"),
    // }
}

// struct Product {
//     name: String,
//     category: ProductCategory,
//     price: f32,
//     in_stock: bool,
// }

// enum ProductCategory {
//     Books,
//     Clothing,
//     Electronics,
// }

// fn main() {
//     let category = ProductCategory::Electronics;
//     let product = Product {
//         name: String::from("tv set"),
//         category,
//         price: 250.0,
//         in_stock: true,
//     };
// }

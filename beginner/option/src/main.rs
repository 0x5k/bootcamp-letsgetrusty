fn main() {
    let username = get_username(1);
    // if let syntax used to handel one case of the enum, if the enum is not the case, the code will not run
    if let Some(name) = username {
        println!("{name}");
    }

    // match syntax used to handel multiple cases of the enum, if the enum is not the case,
    // the code will return None, wich is the default value of the enum
    // match is more flexible than if let, it can be used to handel multiple cases of the enum
    //
    // match username {
    //     Some(name) => println!("{name}"),
    //     None => {}
    // }
}

fn get_username(user_id: u32) -> Option<String> {
    let db_result = String::from("Rust");
    if user_id == 1 { Some(db_result) } else { None }
}

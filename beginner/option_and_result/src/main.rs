fn main() {
    let username = get_username(1);
    if let Some(name) = username {
        println!("{name}");
    }
    // match username {
    //     Some(name) => println!("{name}"),
    //     None => {}
    // }
}
fn get_username(user_id: u32) -> Option<String> {
    let query = format!("GET username FROM users WHERE id={user_id}");
    let db_result = query_db(query);
    db_result.ok()
    // if user_id == 1 {
    //     Some(db_result)
    // } else {
    //     None {}
    // }
}
fn query_db(query: String) -> Result<String, String> {
    if query.is_empty() {
        Err(String::from("db err"))
    } else {
        Ok(String::from("rusty"))
    }
}

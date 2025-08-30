use log::{debug, info};

use derives_macros::Reflective;
#[derive(Debug)]
struct Happy {
    a: i32,
    b: bool,
    c: String,
}

impl Reflective for Happy {
    fn name(&self) -> &'static str {
        "Happy_static"
    }
}

fn parse_config(file: &std) -> AnySrtuct {
    todo!("make a parser for TOML files, what should it give back?")
}

fn main() {
    env_logger::init();

    let happy = Happy {
        a: 1337,
        b: true,
        c: "happy_string".to_string(),
    };
    debug!("there is debug message for {:?}", happy);
    println!("name of struct is {}", happy.name());
    todo!("first to do");
}

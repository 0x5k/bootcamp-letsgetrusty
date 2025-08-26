use log::debug;

#[derive(Debug)]
// #[warn(dead_code)]

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    env_logger::init();
    let leet = 1337;
    let point = Point { x: 3, y: 7 };
    // simple debug logging
    debug!("leet number: {:?}", leet); // output to stderr number 1337
    // debug struct
    debug!("point on map: {:?}", point);
    // multiple arguments
    debug!("values: number={:?}, and point={:?}", leet, point);
    println!("print leet number via println! macro: {}", leet); // otputs to stdout
}

// run it: RUST_LOG=debug cargo run

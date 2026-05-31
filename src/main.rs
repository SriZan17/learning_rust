use std::thread::sleep;
use std::time::Duration;
fn main() {
    println!("Hello, world!");
    let time = Duration::from_secs(7);
    sleep(time);
}

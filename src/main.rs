use std::fs;
fn main() {
    let file = fs::read_to_string("/sys/bus/w1/devices/28-3c01f0952927/w1_slave").unwrap();
    println!("{}", file);
}

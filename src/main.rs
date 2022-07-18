use std::fs;
extern crate regex;
use regex::Regex;
fn main() {
    let file = fs::read_to_string("/sys/bus/w1/devices/28-3c01f0952927/w1_slave").unwrap();
    let re = Regex::new(r"t=\d+").unwrap();
    let caps = re.captures(&file).unwrap();
    let temp = caps.at(0).unwrap().replace("t=", "");
    let temp = temp.parse::<f64>().unwrap() / 1000.;
    println!("{}", temp);
}

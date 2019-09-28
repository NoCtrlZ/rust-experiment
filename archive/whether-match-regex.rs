extern crate regex;

use regex::Regex;

fn main() {
    let re = Regex::new(r"^[0-9]+$").unwrap();
    println!("{}", re.is_match("2014-01-01"));
    println!("{}", re.is_match("2014"));
    println!("{}", re.is_match("h32"));
    println!("{}", re.is_match("h4"));
    println!("{}", re.is_match("32o"));
    println!("{}", re.is_match("0333l"));
}
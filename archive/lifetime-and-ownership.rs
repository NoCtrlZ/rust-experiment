// use std::fs;

// fn sentence_split<'a>(sentence: String) -> Vec<&'a str> {
//     let v: Vec<&str> = sentence.split(" ").collect();
//     v
// }

// fn main() {
//     let s0 = fs::read_to_string("/dev/stdin").unwrap();
//     let s = sentence_split(s0);
//     println!("{:?}", s)
// }

use std::fs;

fn sentence_split(sentence: &str) -> Vec<&str> {
    sentence.split(" ").collect()
}

fn main() {
    let s0 = fs::read_to_string("/dev/stdin").unwrap();
    let s = sentence_split(&s0);
    println!("{:?}", s)
}
use std::fs;

// fn main() {
//     let content = fs::read_to_string("/dev/stdin");
//     println!("{}", content);
// }

// read_to_string function return the Result<String, io::Error> type value

fn main() {
    let content = fs::read_to_string("/dev/stdin").unwrap();
    println!("{}", content);
}

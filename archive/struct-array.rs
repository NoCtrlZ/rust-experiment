struct Token {
    typ: String,
    val: String,
}

fn main() {
    let mut array = Vec::new();
    let x: String = "Number".to_string();
    let y: String = "56".to_string();
    let token = Token{ typ: x, val: y};
    array.push(token);

    let a: String = "String".to_string();
    let b: String = "let".to_string();
    let token2 = Token{ typ: a, val: b};
    array.push(token2);

    println!("{}", array[0].typ);
    println!("{}", array[1].typ);
}
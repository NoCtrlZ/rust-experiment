struct Token {
    typ: String,
    val: String,
}

fn main() {
    let x: String = "Number".to_string();
    let y: String = "56".to_string();
    let token = Token{ typ: x, val: y};

    println!("{} {}", token.typ, token.val);
}
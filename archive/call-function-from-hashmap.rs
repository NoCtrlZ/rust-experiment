use std::collections::HashMap;

type Callback = fn(String);

struct EventHandler {
    user_function: HashMap<String, Callback>,
}


impl EventHandler {
    fn add_user_function(&mut self, name: String, func: Callback) {
        self.user_function.insert(name, func);
    }
}

fn script_foo(_arg: String) {
    println!("hello world {}", _arg);
}

fn main() {
    let mut handler = EventHandler { user_function: HashMap::new() };
	handler.add_user_function("CallFoo".to_string(), script_foo);
	handler.user_function["CallFoo"]("shinsaku".to_string());
}

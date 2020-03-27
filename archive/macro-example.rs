#[macro_use]
extern crate crunchy;

macro_rules! a_macro {
    () => (
        println!("this is a macro");
    )
}

macro_rules! x_and_y {
    (x => $e:expr) => (println!("X: {}", $e));
    (y => $e:expr) => (println!("Y: {}", $e));
}

macro_rules! build_fn {
    ($func_name:ident) => (
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name));
        }
    )
}

fn main() {
    a_macro!();
    x_and_y!(x => 10);
    x_and_y!(y => 20 + 30);
    build_fn!(hi);
    hi();
    unroll! {
        for i in 0..10 {
            println!("hello {}", i);
        }
    }
}
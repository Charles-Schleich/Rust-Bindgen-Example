
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    println!("Hello, world!");
    println!("{}",unsafe{subtract(1,2)});
    println!("{}",unsafe{add(1,2)});
    println!("{}",unsafe{multiply(4,2)});
}
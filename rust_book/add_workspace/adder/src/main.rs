use add_one;

fn main() {
    let num = 10;
    println!("multi-crate workspace: Hello, world! {num} plus one is {}!", add_one::add_one(num));
}
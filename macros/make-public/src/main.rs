use make_public_macro::public_struct; 
use make_public_macro::public; 

#[public_struct]
struct Example1 {
    first: String, 
    pub second: u32, 
}

#[derive(Debug)]
#[public]
struct Example {
    first: String,
    pub second: u32,
}

#[public]
#[derive(Debug)]
struct UnnamedExample(String, f64);

#[public]
enum AnEnumExample {
    First,
    Second,
}


#[public]
struct EmptyStruct {}

fn main() {
    let _e = Example {
        first: "first".to_string(),
        second: 5,
    };
    let u = UnnamedExample("first".to_string(), 5.2);
    println!("{:?}", u);
    let _a = AnEnumExample::First;
    let _empty = EmptyStruct {};
}
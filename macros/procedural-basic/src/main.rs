#[macro_use]
extern crate procedural_basic_macro; 

#[derive(Hello)]
struct Example; 

#[derive(Hello)]
enum Pet {
    Cat, 
}

fn main() {
    let e = Example {}; 
    e.hello_world(); 
    let c = Pet::Cat;
    c.hello_world();
}
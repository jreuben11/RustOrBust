use crate::module1::submodule1::Dummy;

pub mod module1;

fn main() {
    
    let d = Dummy{};
    println!("Hello, {:?}!", d);
}

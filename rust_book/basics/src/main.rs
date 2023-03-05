

pub mod module1; // looks for file as not inline 


fn main() {
    crate::modules::f(); //CH7
}

mod modules {
    use crate::module1::submodule1::Dummy;
    use basics::customer; // prefix with package name as it is in lib.rs

    pub fn f() {
        let d = Dummy{};
        println!("Hello, {:?}!", d);

        customer::eat_at_restaurant();
    }
}



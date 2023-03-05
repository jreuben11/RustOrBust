


pub mod module1; // looks for file as not inline 


fn main() {
    crate::modules::f(); 

    foundation::variables_and_mutability();
    foundation::data_types();
    foundation::expressions_and_functions();
    foundation::control_flow();
}

mod modules { // CH7
    use crate::module1::submodule1::Dummy;
    use basics::customer; // prefix with package name as it is in lib.rs

    pub fn f() {
        let d = Dummy{};
        println!("Hello, {:?}!", d);

        customer::eat_at_restaurant();
    }
}

mod foundation { //CH3
    pub fn variables_and_mutability() {
        let mut x = 5;
        println!("The value of x is: {x}");
        x = 6;
        println!("The value of x is: {x}");

        const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

        //shadowing:
        let y = 5;
        let y = y + 1;
        {
            let y = y * 2;
            println!("The value of y in the inner scope is: {y}");
        }
        println!("The value of y is: {y}");
    }

    pub fn data_types() {
        let guess: u32 = "42".parse().expect("Not a number!");

        let tup: (i32, f64, u8) = (500, 6.4, 1);
        let (x, y, z) = tup;
        let five_hundred = tup.0;
        let six_point_four = tup.1;
        let one = tup.2;

        let a: [i32; 5] = [1, 2, 3, 4, 5];
        println!("Please enter an array index.");

        let mut index = String::new();
        use std::io;
        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");
    
        let index: usize = index
            .trim()
            .parse()
            .expect("Index entered was not a number");
    
        let element = a[index]; //UNSAFE
    
        println!("The value of the element at index {index} is: {element}"); 

    }

    pub fn expressions_and_functions() {
        let y = {
            let x = 3;
            x + 1
        };
        println!("The value of y is: {y}");

        let x = plus_one(5);
        println!("The value of x is: {x}");
    }

    fn plus_one(x: i32) -> i32 {
        x + 1
    }

    pub fn control_flow() {
        let mut count = 0;
        'counting_up: loop {
            println!("count = {count}");
            let mut remaining = 10;
    
            loop {
                println!("remaining = {remaining}");
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'counting_up;
                }
                remaining -= 1;
            }
    
            count += 1;
        }
        println!("End count = {count}");

        for number in (1..4).rev() {
            println!("{number}!");
        }
        println!("LIFTOFF!!!");
    }
}



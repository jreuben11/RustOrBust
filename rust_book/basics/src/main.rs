


pub mod module1; // looks for file as not inline 


fn main() {
    crate::modules::f(); 

    foundation::variables_and_mutability();
    foundation::data_types();
    foundation::expressions_and_functions();
    foundation::control_flow();

    ownership::ownership_examples();
    ownership::borrowing();
    ownership::slices();
}

mod modules { // CH07
    use crate::module1::submodule1::Dummy;
    use basics::customer; // prefix with package name as it is in lib.rs

    pub fn f() {
        let d = Dummy{};
        println!("Hello, {:?}!", d);

        customer::eat_at_restaurant();
    }
}

mod foundation { //CH03
    pub fn variables_and_mutability() {
        let mut x = 5;
        println!("The value of x is: {x}");
        x = 6;
        println!("The value of x is: {x}");

        const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
        let z = THREE_HOURS_IN_SECONDS;
        println!("{}",z);

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
        let _guess: u32 = "42".parse().expect("Not a number!");

        let tup: (i32, f64, u8) = (500, 6.4, 1);
        let (_x, _y, _z) = tup;
        let _five_hundred = tup.0;
        let _six_point_four = tup.1;
        let _one = tup.2;

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

mod ownership { //CH04

    pub fn ownership_examples () {
        // string - goes on heap
        let mut s1 = String::from("hello");
        s1.push_str(", bob!"); 
        let s2 = s1; // take ownership of heap value
        let _s3 = s2.clone();
        doesnt_take_ownership(&s2);
        println!("{}", s2); 
        takes_ownership(s2);

        // i32 - goes on stack
        let x = 5;
        let y = x; // copies value
        makes_copy(x);
        println!("x = {}, y = {}", x, y);

        let mut s3 = gives_ownership(); //also shadow s3
        s3 = takes_and_gives_back(s3);
        println!("{}", s3);

        let (s4, len) = calculate_length(s3); // takes ownership of s3
        println!("The length of '{}' is {}.", s4, len);

    }
    fn takes_ownership(some_string: String) { 
        println!("{}", some_string);
    }
    fn doesnt_take_ownership(some_string: &str) { 
        println!("{}", some_string);
    }
    fn makes_copy(some_integer: i32) { 
        println!("{}", some_integer);
    } 
    fn gives_ownership() -> String {            
        let some_string = String::from("yours"); 
        some_string                            
    }
    fn takes_and_gives_back(a_string: String) -> String { 
        a_string
    }
    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len(); 
        (s, length)
    }

    pub fn borrowing(){
        let mut s = String::from("hello");
        let len = calculate_length2(&s); //refers to s1 but does not own it

        change(&mut s); //pass in a mutable reference
        println!("The length of '{}' was {}.", s, len);

        let r1 = &s; // no problem
        let r2 = &s; // no problem
        println!("{} and {}", r1, r2);
        // variables r1 and r2 will not be used after this point
        {
            let _r3 = &mut s;
        } // r1 goes out of scope here, so we can make a new reference with no problems.
        let _r4 = &mut s;
    }
    fn calculate_length2(s: &String) -> usize {
        s.len()
    }
    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }

    pub fn slices(){
        let mut s = String::from("hello world");
        let _word = first_word_size(&s); // word will get the value 5

        let _hello = &s[0..5];
        let _world = &s[6..11];

        let len = s.len();
        let _slice = &s[3..len];
        let _slice = &s[3..];
        let slice = &s[..];

        let word = first_word(&s);
        let _word2 = first_word(slice);
        let _word3 = first_word("xxx yyy");
        println!("the first word is: {}", word);
        s.clear(); // this empties the String, making it equal to ""

        let a = [1, 2, 3, 4, 5];
        let slice2 = &a[1..3]; // i32 array slice
        assert_eq!(slice2, &[2, 3]);

    }
    fn first_word_size(s: &String) -> usize {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }
        s.len()
    }
    fn first_word(s: &str) -> &str { // param deref coercion: can pass a slice of the String or a reference to the String
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }
}

#![allow(dead_code)]
#![allow(unused_variables)]
// cargo fix --lib -p basics     
// cargo fix --bin "basics"  

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

    structs::use_structs();

    pattern_matching::usage();

    common_collections::vectors();
    common_collections::strings();
    common_collections::hashmaps();

    error_handling::dont_panic();

    generics::data_types();
    generics::traits();
    generics::lifetimes();
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

mod structs { //CH05

    #[derive(Debug)]
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
    struct Point(i32, i32, i32); // tuple struct
    struct AlwaysEqual; // unit-like struct

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
        fn square(size: u32) -> Self { // associated function
            Self {
                width: size,
                height: size,
            }
        }
    }

    pub fn use_structs(){
        let mut user1 = User {
            active: true,
            username: String::from("someusername123"),
            email: String::from("someone@example.com"),
            sign_in_count: 1,
        };
        user1.email = String::from("anotheremail@example.com");

        let user2 = build_user("blah".to_string(), "blah".to_string());
        let user3 = User {
            email: String::from("another@example.com"),
            ..user2 // struct update syntax
        };
        println!("{}-{}-{}", user3.active, user3.username, user3.sign_in_count);

        let _origin = Point(0, 0, 0);
        let _subject = AlwaysEqual;

        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        println!(
            "The area of the rectangle is {} square pixels.",
            rect1.area()
        );
        println!("rect1 is {:?}", rect1);
        println!("rect1 is {:#?}", rect1);
        dbg!(&rect1);

        let rect2 = Rectangle {
            width: 60,
            height: 45,
        };
        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

        let _square = Rectangle::square(10);

    }
    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username, // field init shorthand
            email,
            sign_in_count: 1,
        }
    }
    
}

mod pattern_matching { //CH06
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    pub fn usage(){
        let m = Message::Write(String::from("hello"));
        m.call();

        let _some_number = Some(5);
        let _absent_number: Option<i32> = None;

        value_in_cents(Coin::Penny);
        value_in_cents(Coin::Quarter(UsState::Alaska));

        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);

        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            _ => (),
        }
        fn add_fancy_hat() {}
        fn remove_fancy_hat() {}

        let config_max = Some(3u8);
        if let Some(max) = config_max {
            println!("The maximum is configured to be {}", max);
        }
    }

    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    
}

mod common_collections { //CH08
    pub fn vectors(){
        let mut v: Vec<i32> = Vec::new();
        let v2 = vec![1, 2, 3];
        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);

        let third: &i32 = &v[2];
        println!("The third element is {third}");

        let third: Option<&i32> = v.get(2);
        match third {
            Some(third) => println!("The third element is {third}"),
            None => println!("There is no third element."),
        }

        for i in &v2 {
            println!("{i}");
        }

        for i in &mut v {
            *i += 50; //dereference
        }

        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }
        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];

    }

    pub fn strings(){
        let mut s = String::new();
        s.push_str("bar");
        let s = "initial contents".to_string();
        let s = String::from("initial contents");

        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2; // s1  moved here -> can no longer be used

        for c in "Зд".chars() {
            println!("{c}");
        }
        for b in "Зд".bytes() {
            println!("{b}");
        }
    }

    pub fn hashmaps(){
        use std::collections::HashMap;

        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        let team_name = String::from("Blue");
        let score = scores.get(&team_name).copied().unwrap_or(0);
        for (key, value) in &scores {
            println!("{key}: {value}");
        }
        scores.insert(String::from("Blue"), score + 1);
        scores.entry(String::from("Blue")).or_insert(50);

        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");
        let mut map = HashMap::new();
        map.insert(field_name, field_value);
        // field_name and field_value are invalid at this point

        let text = "hello world wonderful world";

        let mut word_count = HashMap::new();
        for word in text.split_whitespace() {
            let count = word_count.entry(word).or_insert(0);
            *count += 1;
        }
        println!("{:?}", word_count);

    }
}

mod error_handling { //CH09
    pub fn dont_panic(){
        // panic!("crash and burn");
        // let v = vec![1, 2, 3];
        // v[99];

        use std::fs::{self,File,OpenOptions};
        use std::io::{self, Read};

        let file_name = "hello.txt";
        let file_result = OpenOptions::new().write(true).create(true).open(file_name);
        let greeting_file = match file_result {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };

        let greeting_file = File::open(file_name)
            .expect("hello.txt should be included in this project");
        
        fn read_username_from_file() -> Result<String, io::Error> {
            let mut username = String::new();
            File::open("hello.txt")?.read_to_string(&mut username)?;
            Ok(username)
        }
        fn read_username_from_file2() -> Result<String, io::Error> {
            fs::read_to_string("hello.txt")
        }
        let _ = read_username_from_file();

        use std::net::IpAddr;
        let home: IpAddr = "127.0.0.1"
            .parse()
            .expect("Hardcoded IP address should be valid");
    }
}

mod generics { //CH10

    pub fn data_types(){
        let number_list = vec![34, 50, 25, 100, 65];
        let result = largest(&number_list);
        println!("The largest number is {}", result);
    
        let char_list = vec!['y', 'm', 'a', 'q'];
        let result = largest(&char_list);
        println!("The largest char is {}", result);

        fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
            let mut largest = &list[0];
            for item in list {
                if item > largest {
                    largest = item;
                }
            }
            largest
        }
        struct Point<T> {
            x: T,
            y: T,
        }
        impl<T> Point<T> {
            fn x(&self) -> &T {
                &self.x
            }
        }
        impl Point<f32> {
            fn distance_from_origin(&self) -> f32 {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }
        }
        let integer = Point { x: 5, y: 10 };
        let float = Point { x: 1.0, y: 4.0 };
        println!("p.x = {}", integer.x());
        println!("p.distance_from_origin = {}", float.distance_from_origin());
    }
    pub fn traits(){
        pub trait Summary {
            fn summarize(&self) -> String;
            fn summarize2(&self) -> String {
                format!("(Read more from {}...)", self.summarize())
            }
        }
        pub struct NewsArticle {
            pub headline: String,
            pub location: String,
            pub author: String,
            pub content: String,
        }
        
        impl Summary for NewsArticle {
            fn summarize(&self) -> String {
                format!("{}, by {} ({})", self.headline, self.author, self.location)
            }
        }
        
        pub struct Tweet {
            pub username: String,
            pub content: String,
            pub reply: bool,
            pub retweet: bool,
        }
        
        impl Summary for Tweet {
            fn summarize(&self) -> String {
                format!("{}: {}", self.username, self.content)
            }
        }

        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        };
    
        println!("1 new tweet: {}", tweet.summarize());
        println!("blah {}", tweet.summarize2());

        pub fn notify(item: &impl Summary) {
            println!("Breaking news! {}", item.summarize());
        }

        fn returns_summarizable() -> impl Summary {
            Tweet {
                username: String::from("horse_ebooks"),
                content: String::from(
                    "of course, as you probably already know, people",
                ),
                reply: false,
                retweet: false,
            }
        }
        let t = returns_summarizable();
        notify(&t);
    }

    pub fn lifetimes() {
        let string1 = String::from("abcd");
        let string2 = "xyz";
    
        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);

        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }

        struct ImportantExcerpt<'a> {
            part: &'a str,
        }
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ImportantExcerpt {
            part: first_sentence,
        };

        impl<'a> ImportantExcerpt<'a> {
            fn announce_and_return_part(&self, announcement: &str) -> &str {
                println!("Attention please: {}", announcement);
                self.part
            }
        }

        let s: &'static str = "I have a static lifetime.";

        use std::fmt::Display;

        fn longest_with_an_announcement<'a, T>(
            x: &'a str,
            y: &'a str,
            ann: T,
        ) -> &'a str
        where
            T: Display,
        {
            println!("Announcement! {}", ann);
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
    }
    
}

mod testables { // for unit testing:
    #[derive(Debug)]
    pub struct Rectangle {
        pub width: u32,
        pub height: u32,
    }
    impl Rectangle {
        pub fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
    pub fn greeting(name: &str) -> String {
        format!("Hello {}!", name)
    }
    pub fn dont_panic(){
        panic!("AAAAGH!");
    }
    pub struct Guess {
        value: i32,
    }
    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 {
                panic!(
                    "Guess value must be greater than or equal to 1, got {}.",
                    value
                );
            } else if value > 100 {
                panic!(
                    "Guess value must be less than or equal to 100, got {}.",
                    value
                );
            }
    
            Guess { value }
        }
    }
}

#[cfg(test)]
mod tests { //CH11
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works2() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    use super::testables::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"),
        "Greeting did not contain name, value was `{}`",
        result);
    }

    #[test]
    #[should_panic]
    fn should_panic(){
        dont_panic();
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
}
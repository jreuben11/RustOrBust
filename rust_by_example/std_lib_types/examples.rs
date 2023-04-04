

fn main() {
    stack_and_heap::mem_sizes();
    vectors();
    strings::basics();
    strings::literal_escapes();
    strings::raw_literals();
    strings::byte_arrays();
    options();
    println!("{}", handle_results::op(10.0, 1.0));
    handle_results::op2(10.0, 1.0);
    hashmaps::basics();
    hashmaps::custom_keys();
    hashmaps::hashsets();
    ref_counts::rc();
    ref_counts::arc();
}

mod stack_and_heap {

    #[allow(dead_code)]
    #[derive(Debug, Clone, Copy)]
    struct Point {
        x: f64, // 8 bytes - duh ;)
        y: f64,
    }

    // A Rectangle can be specified by where its top left and bottom right 
    // corners are in space
    #[allow(dead_code)]
    struct Rectangle {
        top_left: Point,
        bottom_right: Point,
    }

    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn boxed_origin() -> Box<Point> {
        // Allocate this point on the heap, and return a pointer to it
        Box::new(Point { x: 0.0, y: 0.0 })
    }

    pub fn mem_sizes() {
        use std::mem;
        println!("\nSTACK AND HEAP - MEM SIZES:");
        // (all the type annotations are superfluous)
        // Stack allocated variables
        let point: Point = origin();
        let rectangle: Rectangle = Rectangle {
            top_left: origin(),
            bottom_right: Point { x: 3.0, y: -4.0 }
        };

        // Heap allocated rectangle
        let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
            top_left: origin(),
            bottom_right: Point { x: 3.0, y: -4.0 },
        });

        // The output of functions can be boxed
        let boxed_point: Box<Point> = Box::new(origin());

        // Double indirection
        let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());

        println!("Point occupies {} bytes on the stack",
                mem::size_of_val(&point));
        println!("Rectangle occupies {} bytes on the stack",
                mem::size_of_val(&rectangle));

        // box size == pointer size
        println!("Boxed point occupies {} bytes on the stack",
                mem::size_of_val(&boxed_point));
        println!("Boxed rectangle occupies {} bytes on the stack",
                mem::size_of_val(&boxed_rectangle));
        println!("Boxed box occupies {} bytes on the stack",
                mem::size_of_val(&box_in_a_box));

        // Copy the data contained in `boxed_point` into `unboxed_point`
        let unboxed_point: Point = *boxed_point;
        println!("Unboxed point occupies {} bytes on the stack",
                mem::size_of_val(&unboxed_point));
    }
}

fn vectors() {
    println!("\nVECTORS:");
    // Iterators can be collected into vectors
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into: {:?}", collected_iterator);

    // The `vec!` macro can be used to initialize a vector
    let mut xs = vec![1i32, 2, 3];
    println!("Initial vector: {:?}", xs);

    // Insert new element at the end of the vector
    println!("Push 4 into the vector");
    xs.push(4);
    println!("Vector: {:?}", xs);

    // Error! Immutable vectors can't grow
    // collected_iterator.push(0);
    // FIXME ^ Comment out this line

    // The `len` method yields the number of elements currently stored in a vector
    println!("Vector length: {}", xs.len());

    // Indexing is done using the square brackets (indexing starts at 0)
    println!("Second element: {}", xs[1]);

    // `pop` removes the last element from the vector and returns it
    println!("Pop last element: {:?}", xs.pop());

    // Out of bounds indexing yields a panic
   //  println!("Fourth element: {}", xs[3]);
    // FIXME ^ Comment out this line

    // `Vector`s can be easily iterated over
    println!("Contents of xs:");
    for x in xs.iter() {
        println!("> {}", x);
    }

    // A `Vector` can also be iterated over while the iteration
    // count is enumerated in a separate variable (`i`)
    for (i, x) in xs.iter().enumerate() {
        println!("In position {} we have value {}", i, x);
    }

    // Thanks to `iter_mut`, mutable `Vector`s can also be iterated
    // over in a way that allows modifying each value
    for x in xs.iter_mut() {
        *x *= 3;
    }
    println!("Updated vector: {:?}", xs);
}

mod strings {
    pub fn basics() {
        // : &'static str = ""; // in RO memory
        // : String = String::from(""); // on heap

        println!("\nSTRINGS - BASICS:");
        // (all the type annotations are superfluous)
        // A reference to a string allocated in read only memory
        let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
        println!("Pangram: {}", pangram);
    
        // Iterate over words in reverse, no new string is allocated
        println!("Words in reverse");
        for word in pangram.split_whitespace().rev() {
            println!("> {}", word);
        }
    
        // Copy chars into a vector, sort and remove duplicates
        let mut chars: Vec<char> = pangram.chars().collect();
        chars.sort();
        chars.dedup();
    
        // Create an empty and growable `String`
        let mut string = String::new();
        for c in chars {
            // Insert a char at the end of string
            string.push(c);
            // Insert a string at the end of string
            string.push_str(", ");
        }
    
        // The trimmed string is a slice to the original string, hence no new
        // allocation is performed
        let chars_to_trim: &[char] = &[' ', ','];
        let trimmed_str: &str = string.trim_matches(chars_to_trim); // ???
        println!("Used characters: {}", trimmed_str);
    
        // Heap allocate a string
        let alice = String::from("I like dogs");
        // Allocate new memory and store the modified string there
        let bob: String = alice.replace("dog", "cat");
    
        println!("Alice says: {}", alice); // println macro borrows an implicitly coerced slice
        println!("Bob says: {}", bob);

        // swapping:
        let mut s1: &str = "xxx";
        let mut s2: String = String::from("yyy");
        let s3: &str = &s2;
        println!("{s1}-{s2}-{s3}");
        let t1 = s1;
        let t2 = s2.clone();
        println!("{s1}-{s2}");
        s1 = &t2;
        s2 = t1.to_string();
        println!("{s1}-{s2}");
    }

    pub fn literal_escapes() {
        println!("\nSTRINGS - LITERAL ESCAPES:");
        // You can use escapes to write bytes by their hexadecimal values...
        let byte_escape = "I'm writing \x52\x75\x73\x74!";
        println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);
    
        // ...or Unicode code points.
        let unicode_codepoint = "\u{211D}";
        let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
    
        println!("Unicode character {} (U+211D) is called {}",
                    unicode_codepoint, character_name );
    
    
        let long_string = "String literals
                            can span multiple lines.
                            The linebreak and indentation here ->\
                            <- can be escaped too!";
        println!("{}", long_string);
    }

    pub fn raw_literals() {
        println!("\nSTRINGS - RAW LITERALS:");
        let raw_str = r"Escapes don't work here: \x3F \u{211D}";
        println!("{}", raw_str);
    
        // If you need quotes in a raw string, add a pair of #s
        let quotes = r#"And then I said: "There is no escape!""#;
        println!("{}", quotes);
    
        // If you need "# in your string, just use more #s in the delimiter.
        // You can use up to 65535 #s.
        let longer_delimiter = r###"A string with "# in it. And even "##!"###;
        println!("{}", longer_delimiter);
    }
    

    pub fn byte_arrays() {
        println!("\nSTRINGS - BYTE ARRAYS:");
        use std::str;
        // Note that this is not actually a `&str`
        let bytestring: &[u8; 21] = b"this is a byte string";

        // Byte arrays don't have the `Display` trait, so printing them is a bit limited
        println!("A byte string: {:?}", bytestring);

        // Byte strings can have byte escapes...
        let escaped = b"\x52\x75\x73\x74 as bytes";
        // ...but no unicode escapes
        // let escaped = b"\u{211D} is not allowed";
        println!("Some escaped bytes: {:?}", escaped);


        // Raw byte strings work just like raw strings
        let raw_bytestring = br"\u{211D} is not escaped here";
        println!("{:?}", raw_bytestring);

        // Converting a byte array to `str` can fail
        if let Ok(my_str) = str::from_utf8(raw_bytestring) {
            println!("And the same as text: '{}'", my_str);
        }

        let _quotes = br#"You can also use "fancier" formatting, \
                        like with normal raw strings"#;

        // Byte strings don't have to be UTF-8
        let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82\xbb"; // "ようこそ" in SHIFT-JIS

        // But then they can't always be converted to `str`
        match str::from_utf8(shift_jis) {
            Ok(my_str) => println!("Conversion successful: '{}'", my_str),
            Err(e) => println!("Conversion failed: {:?}", e),
        };
    }
}



fn options() {
    println!("\nOPTIONS:");
    // An integer division that doesn't `panic!`
    fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
        if divisor == 0 {
            // Failure is represented as the `None` variant
            None
        } else {
            // Result is wrapped in a `Some` variant
            Some(dividend / divisor)
        }
    }

    // This function handles a division that may not succeed
    fn try_division(dividend: i32, divisor: i32) {
        // `Option` values can be pattern matched, just like other enums
        match checked_division(dividend, divisor) {
            None => println!("{} / {} failed!", dividend, divisor),
            Some(quotient) => {
                println!("{} / {} = {}", dividend, divisor, quotient)
            },
        }
    }

    try_division(4, 2);
    try_division(1, 0);

    // Binding `None` to a variable needs to be type annotated
    let _none: Option<i32> = None;
    let _equivalent_none = None::<i32>;

    let optional_float = Some(0f32);

    // Unwrapping a `Some` variant will extract the value wrapped.
    println!("{:?} unwraps to {:?}", optional_float, optional_float.unwrap());

    // Unwrapping a `None` variant will `panic!`
    // println!("{:?} unwraps to {:?}", _none, _none.unwrap());
}

mod handle_results {
    // Mathematical "errors" we want to catch
    #[derive(Debug)]
    pub enum MathError {
        DivisionByZero,
        NonPositiveLogarithm,
        NegativeSquareRoot,
    }

    pub type MathResult = Result<f64, MathError>;

    pub fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            // This operation would `fail`, instead let's return the reason of
            // the failure wrapped in `Err`
            Err(MathError::DivisionByZero)
        } else {
            // This operation is valid, return the result wrapped in `Ok`
            Ok(x / y)
        }
    }

    pub fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    pub fn ln(x: f64) -> MathResult {
        if x <= 0.0 {
            Err(MathError::NonPositiveLogarithm)
        } else {
            Ok(x.ln())
        }
    }

    // `op(x, y)` === `sqrt(ln(x / y))`
    pub fn op(x: f64, y: f64) -> f64 {
        println!("\nMATCH RETURNED RESULTS - NESTED:");
        // This is a three level match pyramid!
        match div(x, y) {
            Err(why) => panic!("div {:?}", why),
            Ok(ratio) => match ln(ratio) {
                Err(why) => panic!("ln {:?}", why),
                Ok(ln) => match sqrt(ln) {
                    Err(why) => panic!("sqrt {:?}", why),
                    Ok(sqrt) => sqrt,
                },
            },
        }
    }

    // Intermediate function
    fn op_(x: f64, y: f64) -> MathResult {
        // if `div` "fails", then `DivisionByZero` will be `return`ed
        let ratio = div(x, y)?;

        // if `ln` "fails", then `NonPositiveLogarithm` will be `return`ed
        let ln = ln(ratio)?;

        sqrt(ln)
    }

    pub fn op2(x: f64, y: f64) {
        println!("\nMATCH RETURNED RESULTS - FLATTENED WITH ?:");
        match op_(x, y) {
            Err(why) => panic!("{}", match why {
                MathError::NonPositiveLogarithm
                    => "logarithm of non-positive number",
                MathError::DivisionByZero
                    => "division by zero",
                MathError::NegativeSquareRoot
                    => "square root of negative number",
            }),
            Ok(value) => println!("{}", value),
        }
    }
}

mod hashmaps {
    use std::collections::HashMap;

    

    pub fn basics() { 
        println!("\nHASHMAPS - BASICS:");

        fn call(number: &str) -> &str {
            match number {
                "798-1364" => "We're sorry, the call cannot be completed as dialed. 
                    Please hang up and try again.",
                "645-7689" => "Hello, this is Mr. Awesome's Pizza. My name is Fred.
                    What can I get for you today?",
                _ => "Hi! Who is this again?"
            }
        }

        let mut contacts = HashMap::new();

        contacts.insert("Daniel", "798-1364");
        contacts.insert("Ashley", "645-7689");
        contacts.insert("Katie", "435-8291");
        contacts.insert("Robert", "956-1745");

        // Takes a reference and returns Option<&V>
        match contacts.get(&"Daniel") {
            Some(&number) => println!("Calling Daniel: {}", call(number)),
            _ => println!("Don't have Daniel's number."),
        }

        // `HashMap::insert()` returns `None`
        // if the inserted value is new, `Some(value)` otherwise
        contacts.insert("Daniel", "164-6743");

        match contacts.get(&"Ashley") {
            Some(&number) => println!("Calling Ashley: {}", call(number)),
            _ => println!("Don't have Ashley's number."),
        }

        contacts.remove(&"Ashley"); 

        // `HashMap::iter()` returns an iterator that yields 
        // (&'a key, &'a value) pairs in arbitrary order.
        for (contact, &number) in contacts.iter() {
            println!("Calling {}: {}", contact, call(number)); 
        }
    }

    // Eq requires that you derive PartialEq on the type.
    #[derive(PartialEq, Eq, Hash)]
    struct Account<'a>{
        username: &'a str,
        password: &'a str,
    }

    struct AccountInfo<'a>{
        name: &'a str,
        email: &'a str,
    }

    type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;

    fn try_logon<'a>(accounts: &Accounts<'a>,
            username: &'a str, password: &'a str){
        println!("Username: {}", username);
        println!("Password: {}", password);
        println!("Attempting logon...");

        let logon = Account {
            username,
            password,
        };

        match accounts.get(&logon) {
            Some(account_info) => {
                println!("Successful logon!");
                println!("Name: {}", account_info.name);
                println!("Email: {}", account_info.email);
            },
            _ => println!("Login failed!"),
        }
    }

    pub fn custom_keys(){
        println!("\nHASHMAPS - CUSTOM KEYS:");
        let mut accounts: Accounts = HashMap::new();

        let account = Account {
            username: "j.everyman",
            password: "password123",
        };

        let account_info = AccountInfo {
            name: "John Everyman",
            email: "j.everyman@email.com",
        };

        accounts.insert(account, account_info);

        try_logon(&accounts, "j.everyman", "psasword123");

        try_logon(&accounts, "j.everyman", "password123");
    }


    pub fn hashsets() {
        println!("\nHASHSETS:");
        use std::collections::HashSet;

        let mut a: HashSet<i32> = vec![1i32, 2, 3].into_iter().collect();
        let mut b: HashSet<i32> = vec![2i32, 3, 4].into_iter().collect();

        assert!(a.insert(4));
        assert!(a.contains(&4));

        // `HashSet::insert()` returns false if
        // there was a value already present.
        // assert!(b.insert(4), "Value 4 is already in set B!");
        // FIXME ^ Comment out this line

        b.insert(5);

        // If a collection's element type implements `Debug`,
        // then the collection implements `Debug`.
        // It usually prints its elements in the format `[elem1, elem2, ...]`
        println!("A: {:?}", a);
        println!("B: {:?}", b);

        // Print [1, 2, 3, 4, 5] in arbitrary order
        println!("Union: {:?}", a.union(&b).collect::<Vec<&i32>>());

        // This should print [1]
        println!("Difference: {:?}", a.difference(&b).collect::<Vec<&i32>>());

        // Print [2, 3, 4] in arbitrary order.
        println!("Intersection: {:?}", a.intersection(&b).collect::<Vec<&i32>>());

        // Print [1, 5]
        println!("Symmetric Difference: {:?}",
                a.symmetric_difference(&b).collect::<Vec<&i32>>());
    }
}



mod ref_counts {
    pub fn rc() {
        println!("\nREF_COUNT - RC:");
        use std::rc::Rc;
        let rc_examples = "Rc examples".to_string();
        {
            println!("--- rc_a is created ---");
            
            let rc_a: Rc<String> = Rc::new(rc_examples);
            println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));
            
            {
                println!("--- rc_a is cloned to rc_b ---");
                
                let rc_b: Rc<String> = Rc::clone(&rc_a);
                println!("Reference Count of rc_b: {}", Rc::strong_count(&rc_b));
                println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));
                
                // Two `Rc`s are equal if their inner values are equal
                println!("rc_a and rc_b are equal: {}", rc_a.eq(&rc_b));
                
                // We can use methods of a value directly
                println!("Length of the value inside rc_a: {}", rc_a.len());
                println!("Value of rc_b: {}", rc_b);
                
                println!("--- rc_b is dropped out of scope ---");
            }
            
            println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));
            
            println!("--- rc_a is dropped out of scope ---");
        }
        
        // Error! `rc_examples` already moved into `rc_a`
        // And when `rc_a` is dropped, `rc_examples` is dropped together
        // println!("rc_examples: {}", rc_examples);
        // TODO ^ Try uncommenting this line
    }



    pub fn arc() {
        println!("\nREF_COUNT - ARC:");
        use std::time::Duration;
        use std::sync::Arc;
        use std::thread;
        // This variable declaration is where its value is specified.
        let apple = Arc::new("the same apple");

        for _ in 0..10 {
            // Here there is no value specification as it is a pointer to a
            // reference in the memory heap.
            let apple = Arc::clone(&apple);

            thread::spawn(move || {
                // As Arc was used, threads can be spawned using the value allocated
                // in the Arc variable pointer's location.
                println!("{:?}", apple);
            });
        }

        // Make sure all Arc instances are printed from spawned threads.
        thread::sleep(Duration::from_secs(1));
    }
}




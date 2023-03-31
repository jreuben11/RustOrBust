#![allow(unreachable_code)]
#![allow(unused_labels)]

fn main() {
    if_else();
    loops::basic_loop();
    loops::nested_labeled_loops();
    loops::break_early();
    fizzbuzz_while();
    for_and_range::fizzbuzz_for();
    for_and_range::borrowing_for();
    for_and_range::consuming_for();
    for_and_range::mutating_for();
    matches::basic();
    matches::destructuring_tuples();
    matches::destructuring_arrays();
    matches::destructuring_enums();
    matches::dereferencing();
    matches::destructuring_structs();
    matches::guards();
    matches::bindings();
    if_let();
    while_let();

}
fn if_else() {    
    println!("\nIF ELSE:");
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            // This expression returns an `i32`.
            10 * n
        } else {
            println!(", and is a big number, halve the number");

            // This expression must return an `i32` as well.
            n / 2
            // TODO ^ Try suppressing this expression with a semicolon.
        };
    //   ^ Don't forget to put a semicolon here! All `let` bindings need it.

    println!("{} -> {}", n, big_n);
}

mod loops {
    pub fn basic_loop() {
        println!("\nBASIC LOOP:");
        let mut count = 0u32;

        println!("Let's count until infinity!");

        // Infinite loop
        loop {
            count += 1;

            if count == 3 {
                println!("three");

                // Skip the rest of this iteration
                continue;
            }

            println!("{}", count);

            if count == 5 {
                println!("OK, that's enough");

                // Exit this loop
                break;
            }
        }
    }

    pub fn nested_labeled_loops() {
        println!("\nNESTED LABELED LOOPS:");
        'outer: loop {
            println!("Entered the outer loop");

            'inner: loop {
                println!("Entered the inner loop");

                // This would break only the inner loop
                //break;

                // This breaks the outer loop
                break 'outer;
            }

            println!("This point will never be reached");
        }

        println!("Exited the outer loop");
    }

    pub fn break_early() {

        let mut counter = 0;
    
        let result = loop {
            counter += 1;
    
            if counter == 10 {
                break counter * 2;
            }
        };
    
        assert_eq!(result, 20);
    }
}

fn fizzbuzz_while() {
    println!("\nFIZZBUZZ WHILE:");
    // A counter variable
    let mut n = 1;

    // Loop while `n` is less than 101
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // Increment counter
        n += 1;
    }
}

mod for_and_range {
    pub fn fizzbuzz_for() {
        println!("\nFIZZBUZZ FOR:");
        // `n` will take the values: 1, 2, ..., 100 in each iteration
        for n in 1..101 { // alternative syntax: 1..=100
            if n % 15 == 0 {
                println!("fizzbuzz");
            } else if n % 3 == 0 {
                println!("fizz");
            } else if n % 5 == 0 {
                println!("buzz");
            } else {
                println!("{}", n);
            }
        }
    }

    pub fn borrowing_for() {
        println!("\nBORROWING FOR (iter):");

        let names = vec!["Bob", "Frank", "Ferris"];
    
        for name in names.iter() {
            match name {
                &"Ferris" => println!("There is a rustacean among us!"),
                // TODO ^ Try deleting the & and matching just "Ferris"
                _ => println!("Hello {}", name),
            }
        }
        
        println!("names: {:?}", names);
    }

    pub fn consuming_for() {
        println!("\nCONSUMING FOR (into_iter):");

        let names = vec!["Bob", "Frank", "Ferris"];
    
        for name in names.into_iter() {
            match name {
                "Ferris" => println!("There is a rustacean among us!"),
                _ => println!("Hello {}", name),
            }
        }
        
        // println!("names: {:?}", names);
        // FIXME ^ Comment out this line
    }

    pub fn mutating_for() {
        println!("\nMUTATING FOR (iter_mut):");

        let mut names = vec!["Bob", "Frank", "Ferris"];
    
        for name in names.iter_mut() {
            *name = match name {
                &mut "Ferris" => "There is a rustacean among us!",
                _ => "Hello",
            }
        }
    
        println!("names: {:?}", names);
    }

}

mod matches {
    pub fn basic() {
        println!("\nBASIC MATCHES:");
        let number = 13;
        // TODO ^ Try different values for `number`
    
        println!("Tell me about {}", number);
        match number {
            // Match a single value
            1 => println!("One!"),
            // Match several values
            2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
            // TODO ^ Try adding 13 to the list of prime values
            // Match an inclusive range
            13..=19 => println!("A teen"),
            // Handle the rest of cases
            _ => println!("Ain't special"),
            // TODO ^ Try commenting out this catch-all arm
        }
    
        let boolean = true;
        // Match is an expression too
        let binary = match boolean {
            // The arms of a match must cover all the possible values
            false => 0,
            true => 1,
            // TODO ^ Try commenting out one of these arms
        };
    
        println!("{} -> {}", boolean, binary);
    }

    pub fn destructuring_tuples() {
        println!("\nDESTRUCTURING TUPLES:");
        let triple = (0, -2, 3);
        // TODO ^ Try different values for `triple`
    
        println!("Tell me about {:?}", triple);
        // Match can be used to destructure a tuple
        match triple {
            // Destructure the second and third elements
            (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
            (1, ..)  => println!("First is `1` and the rest doesn't matter"),
            (.., 2)  => println!("last is `2` and the rest doesn't matter"),
            (3, .., 4)  => println!("First is `3`, last is `4`, and the rest doesn't matter"),
            // `..` can be used to ignore the rest of the tuple
            _      => println!("It doesn't matter what they are"),
            // `_` means don't bind the value to a variable
        }
    }

    pub fn destructuring_arrays() {
        println!("\nDESTRUCTURING ARRAYS/SLICES:");
        // Try changing the values in the array, or make it a slice!
        let array = [1, -2, 6];
    
        match array {
            // Binds the second and the third elements to the respective variables
            [0, second, third] =>
                println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),
    
            // Single values can be ignored with _
            [1, _, third] => println!(
                "array[0] = 1, array[2] = {} and array[1] was ignored",
                third
            ),
    
            // You can also bind some and ignore the rest
            [-1, second, ..] => println!(
                "array[0] = -1, array[1] = {} and all the other ones were ignored",
                second
            ),
            // The code below would not compile
            // [-1, second] => ...
    
            // Or store them in another array/slice (the type depends on
            // that of the value that is being matched against)
            [3, second, tail @ ..] => println!(
                "array[0] = 3, array[1] = {} and the other elements were {:?}",
                second, tail
            ),
    
            // Combining these patterns, we can, for example, bind the first and
            // last values, and store the rest of them in a single array
            [first, middle @ .., last] => println!(
                "array[0] = {}, middle = {:?}, array[2] = {}",
                first, middle, last
            ),
        }
    }



    pub fn destructuring_enums() {
        println!("\nDESTRUCTURING ENUMS:");

        // `allow` required to silence warnings because only
        // one variant is used.
        #[allow(dead_code)]
        enum Color {
            // These 3 are specified solely by their name.
            Red,
            Blue,
            Green,
            // These likewise tie `u32` tuples to different names: color models.
            RGB(u32, u32, u32),
            HSV(u32, u32, u32),
            HSL(u32, u32, u32),
            CMY(u32, u32, u32),
            CMYK(u32, u32, u32, u32),
        }
        let color = Color::RGB(122, 17, 40);
        // TODO ^ Try different variants for `color`

        println!("What color is it?");
        // An `enum` can be destructured using a `match`.
        match color {
            Color::Red   => println!("The color is Red!"),
            Color::Blue  => println!("The color is Blue!"),
            Color::Green => println!("The color is Green!"),
            Color::RGB(r, g, b) =>
                println!("Red: {}, green: {}, and blue: {}!", r, g, b),
            Color::HSV(h, s, v) =>
                println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
            Color::HSL(h, s, l) =>
                println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
            Color::CMY(c, m, y) =>
                println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
            Color::CMYK(c, m, y, k) =>
                println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                    c, m, y, k),
            // Don't need another arm because all variants have been examined
        }
    }

    pub fn dereferencing() {
        println!("\nDEREFERENCING:");

        // Assign a reference of type `i32`. The `&` signifies there
        // is a reference being assigned.
        let reference = &4;
    
        match reference {
            // If `reference` is pattern matched against `&val`, it results
            // in a comparison like:
            // `&i32`
            // `&val`
            // ^ We see that if the matching `&`s are dropped, then the `i32`
            // should be assigned to `val`.
            &val => println!("Got a value via destructuring: {:?}", val),
        }
    
        // To avoid the `&`, you dereference before matching.
        match *reference {
            val => println!("Got a value via dereferencing: {:?}", val),
        }
    
        // What if you don't start with a reference? `reference` was a `&`
        // because the right side was already a reference. This is not
        // a reference because the right side is not one.
        let _not_a_reference = 3;
    
        // Rust provides `ref` for exactly this purpose. It modifies the
        // assignment so that a reference is created for the element; this
        // reference is assigned.
        let ref _is_a_reference = 3;
    
        // Accordingly, by defining 2 values without references, references
        // can be retrieved via `ref` and `ref mut`.
        let value = 5;
        let mut mut_value = 6;
    
        // Use `ref` keyword to create a reference.
        match value {
            ref r => println!("Got a reference to a value: {:?}", r),
        }
    
        // Use `ref mut` similarly.
        match mut_value {
            ref mut m => {
                // Got a reference. Gotta dereference it before we can
                // add anything to it.
                *m += 10;
                println!("We added 10. `mut_value`: {:?}", m);
            },
        }
    }

    pub fn destructuring_structs() {
        println!("\nDESTRUCTURING STRUCTS:");

        struct Foo {
            x: (u32, u32),
            y: u32,
        }
    
        // Try changing the values in the struct to see what happens
        let foo = Foo { x: (1, 2), y: 3 };
    
        match foo {
            Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),
    
            // you can destructure structs and rename the variables,
            // the order is not important
            Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),
    
            // and you can also ignore some variables:
            Foo { y, .. } => println!("y = {}, we don't care about x", y),
            // this will give an error: pattern does not mention field `x`
            //Foo { y } => println!("y = {}", y),
        }
    }

    pub fn guards(){
        println!("\nMATCH GUARDS:");

        #[allow(dead_code)]
        enum Temperature {
            Celsius(i32),
            Fahrenheit(i32),
        }
    
        let temperature = Temperature::Celsius(35);
        // ^ TODO try different values for `temperature`
    
        match temperature {
            Temperature::Celsius(t) if t > 30 => println!("{}C is above 30 Celsius", t),
            // The `if condition` part ^ is a guard
            Temperature::Celsius(t) => println!("{}C is below 30 Celsius", t),
    
            Temperature::Fahrenheit(t) if t > 86 => println!("{}F is above 86 Fahrenheit", t),
            Temperature::Fahrenheit(t) => println!("{}F is below 86 Fahrenheit", t),
        }

        let number: u8 = 4;
        match number {
            i if i == 0 => println!("Zero"),
            i if i > 0 => println!("Greater than zero"),
            _ => unreachable!("Should never happen."),
        }
    }

    pub fn bindings(){
        println!("\nMATCH BINDINGS:");
        // A function `age` which returns a `u32`.
        fn age() -> u32 {
            15
        }

        println!("Tell me what type of person you are");

        match age() {
            0             => println!("I haven't celebrated my first birthday yet"),
            // Could `match` 1 ..= 12 directly but then what age
            // would the child be? Instead, bind to `n` for the
            // sequence of 1 ..= 12. Now the age can be reported.
            n @ 1  ..= 12 => println!("I'm a child of age {:?}", n),
            n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
            // Nothing bound. Return the result.
            n             => println!("I'm an old person of age {:?}", n),
        }

        fn some_number() -> Option<u32> {
            Some(42)
        }
        match some_number() {
            // Got `Some` variant, match if its value, bound to `n`,
            // is equal to 42.
            Some(n @ 42) => println!("The Answer: {}!", n),
            // Match any other number.
            Some(n)      => println!("Not interesting... {}", n),
            // Match anything else (`None` variant).
            _            => (),
        }
    }
}

fn if_let() {
    println!("\nIF LET:");
    // All have type `Option<i32>`
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // The `if let` construct reads: "if `let` destructures `number` into
    // `Some(i)`, evaluate the block (`{}`).
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    // If you need to specify a failure, use an else:
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        // Destructure failed. Change to the failure case.
        println!("Didn't match a number. Let's go with a letter!");
    }

    // Provide an altered failing condition.
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    // Destructure failed. Evaluate an `else if` condition to see if the
    // alternate failure branch should be taken:
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        // The condition evaluated false. This branch is the default:
        println!("I don't like letters. Let's go with an emoticon :)!");
    }

    // Our example enum
    enum Foo {
        Bar,
        Baz,
        Qux(u32)
    }

    // Create example variables
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);
    
    // Variable a matches Foo::Bar
    if let Foo::Bar = a {
        println!("a is foobar");
    }
    
    // Variable b does not match Foo::Bar
    // So this will print nothing
    if let Foo::Bar = b {
        println!("b is foobar");
    }
    
    // Variable c matches Foo::Qux which has a value
    // Similar to Some() in the previous example
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    // Binding also works with `if let`
    if let Foo::Qux(_value @ 100) = c {
        println!("c is one hundred");
    }

    use std::str::FromStr;

    #[allow(non_fmt_panics)]
    fn get_count_item(s: &str) -> (u64, &str) {
        let mut it = s.split(' ');
        let (count_str, item) = match (it.next(), it.next()) {
            (Some(count_str), Some(item)) => (count_str, item),
            _ => panic!("Can't segment count item pair: '{s}'"),
        };
        println!("\nIF LET ELSE:");
        let count = if let Ok(count) = u64::from_str(count_str) {
            count
        } else {
            panic!("Can't parse integer: '{count_str}'");
        };
        (count, item)
    }
    assert_eq!(get_count_item("3 chairs"), (3, "chairs"));
}


fn while_let() {
    println!("\nWHILE LET:");
    // Make `optional` of type `Option<i32>`
    let mut optional = Some(0);

    // This reads: "while `let` destructures `optional` into
    // `Some(i)`, evaluate the block (`{}`). Else `break`.
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
        // ^ Less rightward drift and doesn't require
        // explicitly handling the failing case.
    }
    // ^ `if let` had additional optional `else`/`else if`
    // clauses. `while let` does not have these.
}








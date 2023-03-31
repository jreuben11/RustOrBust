fn main() {
    // Unlike C/C++, there's no restriction on the order of function definitions
    // We can use this function here, and define it somewhere later
    fizzbuzz::fizzbuzz_to(100);
    methods::call();
    closures::basic();
    closures::capturing();
    /*
        - Fn: the closure uses the captured value by reference (&T)
        - FnMut: the closure uses the captured value by mutable reference (&mut T)
        - FnOnce: the closure uses the captured value by value (T)
    */
    closures::pass_as_param();
    closures::type_anonymity();
    closures::function_substitution();
    closures::returning_closures();
    closures::iterator_any();   // NOTE: iter / into_iter both work for Vec<T>, but only iter worked for Array
    closures::iterator_search();
    higher_order_functions();
    diverging_functions();
}

mod fizzbuzz {
    // Function that returns a boolean value
    fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
        // Corner case, early return
        if rhs == 0 {
            return false;
        }

        // This is an expression, the `return` keyword is not necessary here
        lhs % rhs == 0
    }

    // Functions that "don't" return a value, actually return the unit type `()`
    fn fizzbuzz(n: u32) -> () {
        if is_divisible_by(n, 15) {
            println!("fizzbuzz");
        } else if is_divisible_by(n, 3) {
            println!("fizz");
        } else if is_divisible_by(n, 5) {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // When a function returns `()`, the return type can be omitted from the
    // signature
    pub fn fizzbuzz_to(n: u32) {
        println!("\nFIZZBUZZ:");
        for n in 1..=n {
            fizzbuzz(n);
        }
    }
}

mod methods {
    struct Point {
        x: f64,
        y: f64,
    }
    
    // Implementation block, all `Point` associated functions & methods go in here
    impl Point {
        // This is an "associated function" because this function is associated with
        // a particular type, that is, Point.
        //
        // Associated functions don't need to be called with an instance.
        // These functions are generally used like constructors.
        fn origin() -> Point {
            Point { x: 0.0, y: 0.0 }
        }
    
        // Another associated function, taking two arguments:
        fn new(x: f64, y: f64) -> Point {
            Point { x: x, y: y }
        }
    }
    
    struct Rectangle {
        p1: Point,
        p2: Point,
    }
    
    impl Rectangle {
        // This is a method
        // `&self` is sugar for `self: &Self`, where `Self` is the type of the
        // caller object. In this case `Self` = `Rectangle`
        fn area(&self) -> f64 {
            // `self` gives access to the struct fields via the dot operator
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;
    
            // `abs` is a `f64` method that returns the absolute value of the
            // caller
            ((x1 - x2) * (y1 - y2)).abs()
        }
    
        fn perimeter(&self) -> f64 {
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;
    
            2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
        }
    
        // This method requires the caller object to be mutable
        // `&mut self` desugars to `self: &mut Self`
        fn translate(&mut self, x: f64, y: f64) {
            self.p1.x += x;
            self.p2.x += x;
    
            self.p1.y += y;
            self.p2.y += y;
        }
    }
    
    // `Pair` owns resources: two heap allocated integers
    struct Pair(Box<i32>, Box<i32>);
    
    impl Pair {
        // This method "consumes" the resources of the caller object
        // `self` desugars to `self: Self`
        fn destroy(self) {
            // Destructure `self`
            let Pair(first, second) = self;
    
            println!("Destroying Pair({}, {})", first, second);
    
            // `first` and `second` go out of scope and get freed
        }
    }
    
    pub fn call() {
        println!("\nIMPL METHODS:");
        let rectangle = Rectangle {
            // Associated functions are called using double colons
            p1: Point::origin(),
            p2: Point::new(3.0, 4.0),
        };
    
        // Methods are called using the dot operator
        // Note that the first argument `&self` is implicitly passed, i.e.
        // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
        println!("Rectangle perimeter: {}", rectangle.perimeter());
        println!("Rectangle area: {}", rectangle.area());
    
        let mut square = Rectangle {
            p1: Point::origin(),
            p2: Point::new(1.0, 1.0),
        };
    
        // Error! `rectangle` is immutable, but this method requires a mutable
        // object
        //rectangle.translate(1.0, 0.0);
        // TODO ^ Try uncommenting this line
    
        // Okay! Mutable objects can call mutable methods
        square.translate(1.0, 1.0);
    
        let pair = Pair(Box::new(1), Box::new(2));
    
        pair.destroy();
    
        // Error! Previous `destroy` call "consumed" `pair`
        //pair.destroy();
        // TODO ^ Try uncommenting this line
    }
}

mod closures {
    pub fn basic() {
        println!("\nBASIC CLOSURES:");
        let outer_var = 42;
        
        // A regular function can't refer to variables in the enclosing environment
        //fn function(i: i32) -> i32 { i + outer_var }
        // TODO: uncomment the line above and see the compiler error. The compiler
        // suggests that we define a closure instead.
    
        // Closures are anonymous, here we are binding them to references
        // Annotation is identical to function annotation but is optional
        // as are the `{}` wrapping the body. These nameless functions
        // are assigned to appropriately named variables.
        let closure_annotated = |i: i32| -> i32 { i + outer_var };
        let closure_inferred  = |i     |          i + outer_var  ;
    
        // Call the closures.
        println!("closure_annotated: {}", closure_annotated(1));
        println!("closure_inferred: {}", closure_inferred(1));
        // Once closure's type has been inferred, it cannot be inferred again with another type.
        //println!("cannot reuse closure_inferred with another type: {}", closure_inferred(42i64));
        // TODO: uncomment the line above and see the compiler error.
    
        // A closure taking no arguments which returns an `i32`.
        // The return type is inferred.
        let one = || 1;
        println!("closure returning one: {}", one());
    
    }

    pub fn capturing() {
        println!("\nCLOSURE CAPTURING:");
        use std::mem;
        
        let color = String::from("green");
    
        // A closure to print `color` which immediately borrows (`&`) `color` and
        // stores the borrow and closure in the `print` variable. It will remain
        // borrowed until `print` is used the last time. 
        //
        // `println!` only requires arguments by immutable reference so it doesn't
        // impose anything more restrictive.
        let print = || println!("`color`: {}", color);
    
        // Call the closure using the borrow.
        print();
    
        // `color` can be borrowed immutably again, because the closure only holds
        // an immutable reference to `color`. 
        let _reborrow = &color;
        print();
    
        // A move or reborrow is allowed after the FINAL use of `print`
        let _color_moved = color;
    
    
        let mut count = 0;
        // A closure to increment `count` could take either `&mut count` or `count`
        // but `&mut count` is less restrictive so it takes that. Immediately
        // borrows `count`.
        //
        // A `mut` is required on `inc` because a `&mut` is stored inside. Thus,
        // calling the closure mutates the closure which requires a `mut`.
        let mut inc = || {
            count += 1;
            println!("`count`: {}", count);
        };
    
        // Call the closure using a mutable borrow.
        inc();
    
        // The closure still mutably borrows `count` because it is called later.
        // An attempt to reborrow will lead to an error.
        // let _reborrow = &count; 
        // ^ TODO: try uncommenting this line.
        inc();
    
        // The closure no longer needs to borrow `&mut count`. Therefore, it is
        // possible to reborrow without an error
        let _count_reborrowed = &mut count; 
    
        
        // A non-copy type.
        let movable = Box::new(3);
    
        // `mem::drop` requires `T` so this must take by value. A copy type
        // would copy into the closure leaving the original untouched.
        // A non-copy must move and so `movable` immediately moves into
        // the closure.
        let consume = || {
            println!("`movable`: {:?}", movable);
            mem::drop(movable);
        };
    
        // `consume` consumes the variable so this can only be called once.
        consume();
        // consume();
        // ^ TODO: Try uncommenting this line.
   
        // `Vec` has non-copy semantics.
        let haystack = vec![1, 2, 3];
    
        let contains = move |needle| haystack.contains(needle);
    
        println!("{}", contains(&1));
        println!("{}", contains(&4));
    
        // println!("There're {} elements in vec", haystack.len());
        // ^ Uncommenting above line will result in compile-time error
        // because borrow checker doesn't allow re-using variable after it
        // has been moved.
        
        // Removing `move` from closure's signature will cause closure
        // to borrow _haystack_ variable immutably, hence _haystack_ is still
        // available and uncommenting above line will not cause an error.
    }

    pub fn pass_as_param() {
        println!("\nCLOSURE PASS AS PARAM:");
        // A function which takes a closure as an argument and calls it.
        // <F> denotes that F is a "Generic type parameter"
        fn apply<F>(f: F) where
        // The closure takes no input and returns nothing.
        F: FnOnce() {
        // ^ TODO: Try changing this to `Fn` or `FnMut`.
            f();
        }

        // A function which takes a closure and returns an `i32`.
        fn apply_to_3<F>(f: F) -> i32 where
        // The closure takes an `i32` and returns an `i32`.
        F: Fn(i32) -> i32 {
            f(3)
        }

        use std::mem;
    
        let greeting = "hello";
        // A non-copy type.
        // `to_owned` creates owned data from borrowed one
        let mut farewell = "goodbye".to_owned();
    
        // Capture 2 variables: `greeting` by reference and
        // `farewell` by value.
        let diary = || {
            // `greeting` is by reference: requires `Fn`.
            println!("I said {}.", greeting);
    
            // Mutation forces `farewell` to be captured by
            // mutable reference. Now requires `FnMut`.
            farewell.push_str("!!!");
            println!("Then I screamed {}.", farewell);
            println!("Now I can sleep. zzzzz");
    
            // Manually calling drop forces `farewell` to
            // be captured by value. Now requires `FnOnce`.
            mem::drop(farewell);
        };
    
        // Call the function which applies the closure.
        apply(diary);
    
        // `double` satisfies `apply_to_3`'s trait bound
        let double = |x| 2 * x;
    
        println!("3 doubled: {}", apply_to_3(double));
    }

    

    pub fn type_anonymity() {
        println!("\nCLOSURE TYPE ANONYMITY:");

        // `F` must implement `Fn` for a closure which takes no
        // inputs and returns nothing - exactly what is required
        // for `print`.
        fn apply<F>(f: F) where
        F: Fn() {
            f();
        }

        let x = 7;

        // Capture `x` into an anonymous type and implement
        // `Fn` for it. Store it in `print`.
        let print = || println!("{}", x);

        apply(print);
    }


    pub fn function_substitution() {
        println!("\nCLOSURE FUNCTION SUBSTITUTION:");

        // Define a function which takes a generic `F` argument
        // bounded by `Fn`, and calls it
        fn call_me<F: Fn()>(f: F) {
            f();
        }

        // Define a wrapper function satisfying the `Fn` bound
        fn function() {
            println!("I'm a function!");
        }
        // Define a closure satisfying the `Fn` bound
        let closure = || println!("I'm a closure!");

        call_me(closure);
        call_me(function);
    }
    
    
    pub fn returning_closures() {
        println!("\nRETURNING CLOSURES:");
        fn create_fn() -> impl Fn() {
            let text = "Fn".to_owned();
        
            move || println!("This is a: {}", text)
        }
        
        fn create_fnmut() -> impl FnMut() {
            let text = "FnMut".to_owned();
        
            move || println!("This is a: {}", text)
        }
        
        fn create_fnonce() -> impl FnOnce() {
            let text = "FnOnce".to_owned();
        
            move || println!("This is a: {}", text)
        }

        let fn_plain = create_fn();
        let mut fn_mut = create_fnmut();
        let fn_once = create_fnonce();
    
        fn_plain();
        fn_mut();
        fn_once();
    }

    pub fn iterator_any() {
        println!("\n ITERATOR ANY - ITER/INTO_ITER:");
        let vec1 = vec![1, 2, 3];
        let vec2 = vec![4, 5, 6];
    
        // `iter()` for vecs yields `&i32`. Destructure to `i32`.
        println!("2 in vec1: {}", vec1.iter()     .any(|&x| x == 2));
        // `into_iter()` for vecs yields `i32`. No destructuring required.
        println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));
    
        // `iter()` only borrows `vec1` and its elements, so they can be used again
        println!("vec1 len: {}", vec1.len());
        println!("First element of vec1 is: {}", vec1[0]);
        // `into_iter()` does move `vec2` and its elements, so they cannot be used again
        // println!("First element of vec2 is: {}", vec2[0]);
        // println!("vec2 len: {}", vec2.len());
        // TODO: uncomment two lines above and see compiler errors.
    
        let array1 = [1, 2, 3];
        // let array2 = [4, 5, 6];
    
        // `iter()` for arrays yields `&i32`.
        println!("2 in array1: {}", array1.iter()     .any(|&x| x == 2));
        // `into_iter()` for arrays yields `i32`.
        // println!("2 in array2: {}", array2.into_iter().any(|x| x == 2));
        //                                                          ^^ no implementation for `&{integer} == {integer}`
    }

    pub fn iterator_search() {
        println!("\nITERATOR SEARCH:");

        let vec1 = vec![1, 2, 3];
        let vec2 = vec![4, 5, 6];
    
        // `iter()` for vecs yields `&i32`.
        let mut iter = vec1.iter();
        // `into_iter()` for vecs yields `i32`.
        let mut into_iter = vec2.into_iter();
    
        // `iter()` for vecs yields `&i32`, and we want to reference one of its
        // items, so we have to destructure `&&i32` to `i32`
        println!("Find 2 in vec1: {:?}", iter     .find(|&&x| x == 2));
        // `into_iter()` for vecs yields `i32`, and we want to reference one of
        // its items, so we have to destructure `&i32` to `i32`
        println!("Find 2 in vec2: {:?}", into_iter.find(| &x| x == 2));
    
        let array1 = [1, 2, 3];
        // let array2 = [4, 5, 6];
    
        // `iter()` for arrays yields `&i32`
        println!("Find 2 in array1: {:?}", array1.iter()     .find(|&&x| x == 2));
        // `into_iter()` for arrays yields `i32`
        //println!("Find 2 in array2: {:?}", array2.into_iter().find(|&x| x == 2));

        let vec = vec![1, 9, 3, 3, 13, 2];

        // `iter()` for vecs yields `&i32` and `position()` does not take a reference, so
        // we have to destructure `&i32` to `i32`
        let index_of_first_even_number = vec.iter().position(|&x| x % 2 == 0);
        assert_eq!(index_of_first_even_number, Some(5));
        
        // `into_iter()` for vecs yields `i32` and `position()` does not take a reference, so
        // we do not have to destructure    
        let index_of_first_negative_number = vec.into_iter().position(|x| x < 0);
        assert_eq!(index_of_first_negative_number, None);
    }
}



fn higher_order_functions() {
    println!("\nHIGHER ORDER FUNCTIONS:");
    fn is_odd(n: u32) -> bool {
        n % 2 == 1
    }

    println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 1000;

    // Imperative approach
    // Declare accumulator variable
    let mut acc = 0;
    // Iterate: 0, 1, 2, ... to infinity
    for n in 0.. {
        // Square the number
        let n_squared = n * n;

        if n_squared >= upper {
            // Break loop if exceeded the upper limit
            break;
        } else if is_odd(n_squared) {
            // Accumulate value, if it's odd
            acc += n_squared;
        }
    }
    println!("imperative style: {}", acc);

    // Functional approach
    let sum_of_squared_odd_numbers: u32 =
        (0..).map(|n| n * n)                             // All natural numbers squared
             .take_while(|&n_squared| n_squared < upper) // Below upper limit
             .filter(|&n_squared| is_odd(n_squared))     // That are odd
             .sum();                                     // Sum them
    println!("functional style: {}", sum_of_squared_odd_numbers);
}


fn diverging_functions() {
    println!("\nDIVERGING FUNCTIONS:");

    #[allow(dead_code)]
    fn foo() -> ! {
        panic!("This call never returns.");
    }

    fn some_fn() {
        ()
    }
    let _a: () = some_fn();
    println!("This function returns and you can see this line.");
    
    
    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            // Notice that the return type of this match expression must be u32
            // because of the type of the "addition" variable.
            let addition: u32 = match i%2 == 1 {
                // The "i" variable is of type u32, which is perfectly fine.
                true => i,
                // On the other hand, the "continue" expression does not return
                // u32, but it is still fine, because it never returns and therefore
                // does not violate the type requirements of the match expression.
                false => continue,
            };
            acc += addition;
        }
        acc
    }
    println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
}
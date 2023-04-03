fn main(){
    raii_and_drop();
    ownership::move_ownership();
    ownership::partial_move();
    borrowing::basics();
    borrowing::mutability();
    borrowing::mutability_aliasing();
    borrowing::ref_pattern();
    lifetimes::explicit();
    lifetimes::functions();
    lifetimes::methods();
    lifetimes::structs();
    lifetimes::traits();
    lifetimes::generic_bounds();
    lifetimes::coercion();
    lifetimes::statics();
    lifetimes::ellision();
}


fn raii_and_drop() {
    println!("\nRAII + DROP:");
    fn create_box() {
        // Allocate an integer on the heap
        let _box1 = Box::new(3i32);
    
        // `_box1` is destroyed here, and memory gets freed
    }

    // Allocate an integer on the heap
    let _box2 = Box::new(5i32);

    // A nested scope:
    {
        // Allocate an integer on the heap
        let _box3 = Box::new(4i32);

        // `_box3` is destroyed here, and memory gets freed
    }

    // Creating lots of boxes just for fun
    // There's no need to manually free memory!
    for _ in 0u32..1_000 {
        create_box();
    }

    struct ToDrop;
    impl Drop for ToDrop {
        fn drop(&mut self) {
            println!("ToDrop is being dropped");
        }
    }

    let _x = ToDrop;
    println!("Made a ToDrop!");


    // `_box2` is destroyed here, and memory gets freed
}

mod ownership {
    pub fn move_ownership(){
        println!("\nOWNERSHIP - MOVE:");

        // This function takes ownership of the heap allocated memory
        fn destroy_box(c: Box<i32>) {
            println!("Destroying a box that contains {}", c);

            // `c` is destroyed and the memory freed
        }


        // _Stack_ allocated integer
        let x = 5u32;

        // *Copy* `x` into `y` - no resources are moved
        let y = x;

        // Both values can be independently used
        println!("x is {}, and y is {}", x, y);

        // `a` is a pointer to a _heap_ allocated integer
        let a = Box::new(5i32);

        println!("a contains: {}", a);

        // *Move* `a` into `b`
        let b = a;
        // The pointer address of `a` is copied (not the data) into `b`.
        // Both are now pointers to the same heap allocated data, but
        // `b` now owns it.
        
        // Error! `a` can no longer access the data, because it no longer owns the
        // heap memory
        //println!("a contains: {}", a);
        // TODO ^ Try uncommenting this line

        // This function takes ownership of the heap allocated memory from `b`
        destroy_box(b);

        // Since the heap memory has been freed at this point, this action would
        // result in dereferencing freed memory, but it's forbidden by the compiler
        // Error! Same reason as the previous Error
        //println!("b contains: {}", b);
        // TODO ^ Try uncommenting this line

    }

    pub fn partial_move(){
        println!("\nOWNERSHIP - PARTIAL MOVE:");
        #[derive(Debug)]
        struct Person {
            name: String,
            age: Box<u8>,
        }
    
        let person = Person {
            name: String::from("Alice"),
            age: Box::new(20),
        };
    
        // `name` is moved out of person, but `age` is referenced
        let Person { name, ref age } = person;
    
        println!("The person's age is {}", age);
    
        println!("The person's name is {}", name);
    
        // Error! borrow of partially moved value: `person` partial move occurs
        //println!("The person struct is {:?}", person);
    
        // `person` cannot be used but `person.age` can be used as it is not moved
        println!("The person's age from person struct is {}", person.age);
    }
}

mod borrowing {
    pub fn basics() {
        println!("\nBORROWING BASICS:");
        // This function takes ownership of a box and implicitly destroys it when it goes out of scope
        fn eat_box_i32(boxed_i32: Box<i32>) {
            println!("Destroying box that contains {}", boxed_i32);
        }
        
        // This function borrows an i32
        fn borrow_i32(borrowed_i32: &i32) {
            println!("This int is: {}", borrowed_i32);
        }
    
        // Create a boxed i32, and a stacked i32
        let boxed_i32 = Box::new(5_i32);
        let stacked_i32 = 6_i32;

        // Borrow the contents of the box. Ownership is not taken,
        // so the contents can be borrowed again.
        borrow_i32(&boxed_i32);
        borrow_i32(&stacked_i32);

        {
            // Take a reference to the data contained inside the box
            let _ref_to_i32: &i32 = &boxed_i32;

            // Error!
            // Can't destroy `boxed_i32` while the inner value is borrowed later in scope.
            // eat_box_i32(boxed_i32);
            // FIXME ^ Comment out this line

            // Attempt to borrow `_ref_to_i32` after inner value is destroyed
            borrow_i32(_ref_to_i32);
            // `_ref_to_i32` goes out of scope and is no longer borrowed.
        }

        // `boxed_i32` can now give up ownership to `eat_box` and be destroyed
        eat_box_i32(boxed_i32);
    }

    pub fn mutability() {
        println!("\nBORROWING MUTABILITY:");
        #[allow(dead_code)]
        #[derive(Clone, Copy)]
        // struct Book {
        //     // `&'static str` is a reference to a string allocated in read only memory
        //     author: &'static str,
        //     title: &'static str,
        //     year: u32,
        // }
        struct Book<'a> {
            author: &'a str,
            title: &'a str,
            year: u32,
        }
        // struct Book {
        //     author: String, // ERROR: this field does not implement `Copy`
        //     title: String,  // the trait `Copy` may not be implemented for this type
        //     year: u32,
        // }

        // This function takes a reference to a book
        fn borrow_book(book: &Book) {
            println!("I immutably borrowed {} - {} edition", book.title, book.year);
        }

        // This function takes a reference to a mutable book and changes `year` to 2014
        fn new_edition(book: &mut Book) {
            book.year = 2014;
            println!("I mutably borrowed {} - {} edition", book.title, book.year);
        }

    
        // Create an immutable Book named `immutabook`
        let immutabook = Book {
            // string literals have type `&'static str`
            author: "Douglas Hofstadter",
            title: "Gödel, Escher, Bach",
            year: 1979,
        };

        // Create a mutable copy of `immutabook` and call it `mutabook`
        let mut mutabook = immutabook;
        
        // Immutably borrow an immutable object
        borrow_book(&immutabook);

        // Immutably borrow a mutable object
        borrow_book(&mutabook);
        
        // Borrow a mutable object as mutable
        new_edition(&mut mutabook);
        
        // Error! Cannot borrow an immutable object as mutable
        // new_edition(&mut immutabook);
        // FIXME ^ Comment out this line
    }

    pub fn mutability_aliasing(){
        println!("\nBORROWING MUTABILITY ALIASING:");

        struct Point { x: i32, y: i32, z: i32 }


        let mut point = Point { x: 0, y: 0, z: 0 };

        let borrowed_point = &point;
        let another_borrow = &point;

        // Data can be accessed via the references and the original owner
        println!("Point has coordinates: ({}, {}, {})",
                    borrowed_point.x, another_borrow.y, point.z);

        // Error! Can't  borrow `point` as mutable because it's CURRENTLY
        // borrowed as immutable.
        // let mutable_borrow = &mut point;
        // TODO ^ Try uncommenting this line

        // The borrowed values are used again here
        println!("Point has coordinates: ({}, {}, {})",
                    borrowed_point.x, another_borrow.y, point.z);

        // The immutable references are no longer used for the rest of the code so
        // it is possible to reborrow with a mutable reference.
        let mutable_borrow = &mut point;

        // Change data via mutable reference
        mutable_borrow.x = 5;
        mutable_borrow.y = 2;
        mutable_borrow.z = 1;

        // Error! Can't borrow `point` as immutable because it's currently
        // borrowed as mutable.
        // let y = &point.y;
        // TODO ^ Try uncommenting this line

        // Error! Can't print because `println!` takes an immutable reference.
        // println!("Point Z coordinate is {}", point.z);
        // TODO ^ Try uncommenting this line

        // Ok! Mutable references can be passed as immutable to `println!`
        println!("Point has coordinates: ({}, {}, {})",
                    mutable_borrow.x, mutable_borrow.y, mutable_borrow.z);

        // The mutable reference is no longer used for the rest of the code so it
        // is possible to reborrow
        let new_borrowed_point = &point;
        println!("Point now has coordinates: ({}, {}, {})",
                new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z);

    }

    

    pub fn ref_pattern() {
        println!("\nBORROWING REF PATTERN:");

        #[derive(Clone, Copy)]
        struct Point { x: i32, y: i32 }

        let c = 'Q';

        // A `ref` borrow on the left side of an assignment is equivalent to
        // an `&` borrow on the right side.
        let ref ref_c1 = c;
        let ref_c2 = &c;

        println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

        let point = Point { x: 0, y: 0 };

        // `ref` is also valid when destructuring a struct.
        let _copy_of_x = {
            // `ref_to_x` is a reference to the `x` field of `point`.
            let Point { x: ref ref_to_x, y: _ } = point;

            // Return a copy of the `x` field of `point`.
            *ref_to_x
        };

        // A mutable copy of `point`
        let mut mutable_point = point;

        {
            // `ref` can be paired with `mut` to take mutable references.
            let Point { x: _, y: ref mut mut_ref_to_y } = mutable_point;

            // Mutate the `y` field of `mutable_point` via a mutable reference.
            *mut_ref_to_y = 1;
        }

        println!("point is ({}, {})", point.x, point.y);
        println!("mutable_point is ({}, {})", mutable_point.x, mutable_point.y);

        // A mutable tuple that includes a pointer
        let mut mutable_tuple = (Box::new(5u32), 3u32);
        
        {
            // Destructure `mutable_tuple` to change the value of `last`.
            let (_, ref mut last) = mutable_tuple;
            *last = 2u32;
        }
        
        println!("tuple is {:?}", mutable_tuple);
    }
}

mod lifetimes {
    pub fn explicit() {
        println!("\nLIFETIMES - EXPLICIT:");
        // `print_refs` takes two references to `i32` which have different
        // lifetimes `'a` and `'b`. These two lifetimes must both be at
        // least as long as the function `print_refs`.
        fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
            println!("x is {} and y is {}", x, y);
        }

        // A function which takes no arguments, but has a lifetime parameter `'a`.
        fn failed_borrow<'a>() {
            let _x = 12;

            // ERROR: `_x` does not live long enough
            // let y: &'a i32 = &_x;

            // Attempting to use the lifetime `'a` as an explicit type annotation 
            // inside the function will fail because the lifetime of `&_x` is shorter
            // than that of `y`. A short lifetime cannot be coerced into a longer one.
        }


        // Create variables to be borrowed below.
        let (four, nine) = (4, 9);
        
        // Borrows (`&`) of both variables are passed into the function.
        print_refs(&four, &nine);
        // Any input which is borrowed must outlive the borrower. 
        // In other words, the lifetime of `four` and `nine` must 
        // be longer than that of `print_refs`.
        
        failed_borrow();
        // `failed_borrow` contains no references to force `'a` to be 
        // longer than the lifetime of the function, but `'a` is longer.
        // Because the lifetime is never constrained, it defaults to `'static`.
    }

    pub fn functions() {
        println!("\nLIFETIMES - FUNCTIONS:");

        // One input reference with lifetime `'a` which must live
        // at least as long as the function.
        fn print_one<'a>(x: &'a i32) {
            println!("`print_one`: x is {}", x);
        }

        // Mutable references are possible with lifetimes as well.
        fn add_one<'a>(x: &'a mut i32) {
            *x += 1;
        }

        // Multiple elements with different lifetimes. In this case, it
        // would be fine for both to have the same lifetime `'a`, but
        // in more complex cases, different lifetimes may be required.
        fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
            println!("`print_multi`: x is {}, y is {}", x, y);
        }

        // Returning references that have been passed in is acceptable.
        // However, the correct lifetime must be returned.
        fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 { x }

        // fn pass_x(x: &i32, _: &i32) -> &i32 { x } // ERROR: expected named lifetime parameter: this pattern is not ellided

        //fn invalid_output<'a>() -> &'a String { &String::from("foo") }
        // The above is invalid: `'a` must live longer than the function.
        // Here, `&String::from("foo")` would create a `String`, followed by a
        // reference. Then the data is dropped upon exiting the scope, leaving
        // a reference to invalid data to be returned.


        let x = 7;
        let y = 9;
        
        print_one(&x);
        print_multi(&x, &y);
        
        let z = pass_x(&x, &y);
        print_one(z);

        let mut t = 3;
        add_one(&mut t);
        print_one(&t);
    }

   

    pub fn methods() {
        println!("\nLIFETIMES - METHODS:");
        struct Owner(i32);

        impl Owner {
            // Annotate lifetimes as in a standalone function.
            fn add_one<'a>(&'a mut self) { self.0 += 1; }
            fn print<'a>(&'a self) {
                println!("`print`: {}", self.0);
            }
        }
        
        let mut owner = Owner(18);

        owner.print();
        owner.add_one();
        owner.print();
    }

    pub fn structs(){
        println!("\nLIFETIMES - STRUCTS:");
        // A type `Borrowed` which houses a reference to an
        // `i32`. The reference to `i32` must outlive `Borrowed`.
        #[derive(Debug)]
        struct Borrowed<'a>(&'a i32);

        // Similarly, both references here must outlive this structure.
        #[allow(dead_code)]
        #[derive(Debug)]
        struct NamedBorrowed<'a> {
            x: &'a i32,
            y: &'a i32,
        }

        // An enum which is either an `i32` or a reference to one.
        #[derive(Debug)]
        enum Either<'a> {
            Num(i32),
            Ref(&'a i32),
        }

        let x = 18;
        let y = 15;

        let single = Borrowed(&x);
        let double = NamedBorrowed { x: &x, y: &y };
        let reference = Either::Ref(&x);
        let number    = Either::Num(y); // value type is copied in

        println!("x is borrowed in {:?}", single);
        println!("x and y are borrowed in {:?}", double);
        println!("x is borrowed in {:?}", reference);
        println!("y is *not* borrowed in {:?}", number);

    }

    pub fn traits() {
        println!("\nLIFETIMES - TRAITS:");

        // A struct with annotation of lifetimes.
        #[derive(Debug)]
        struct Borrowed<'a> {
            _x: &'a i32,
        }

        // Annotate lifetimes to impl.
        impl<'a> Default for Borrowed<'a> {
            fn default() -> Self {
                Self {
                    _x: &10,
                }
            }
        }

        let b: Borrowed = Default::default();
        println!("b is {:?}", b);
    }

    pub fn generic_bounds() {
        println!("\nLIFETIMES - GENERIC BOUNDS:");
        use std::fmt::Debug; // Trait to bound with.
    
        #[derive(Debug)]
        struct MyRef<'a, T: 'a>(&'a T);
        // `MyRef` contains a reference to a generic type `T` that has
        // an unknown lifetime `'a`. `T` is bounded such that any
        // *references* in `T` must outlive `'a`. Additionally, the lifetime
        // of `Ref` may not exceed `'a`.
    
        // A generic function which prints using the `Debug` trait.
        fn print<T>(t: T) where
            T: Debug {
            println!("`print`: t is {:?}", t);
        }
    
        // Here a reference to `T` is taken where `T` implements
        // `Debug` and all *references* in `T` outlive `'a`. In
        // addition, `'a` must outlive the function.
        fn print_ref<'a, T>(t: &'a T) where
            T: Debug + 'a {
            println!("`print_ref`: t is {:?}", t);
        }
        let x = 7;
        let ref_x = MyRef(&x);
    
        print_ref(&ref_x);
        print(ref_x);
    }

    pub fn coercion() {
        println!("\nLIFETIMES - COERCION:");
        // Here, Rust infers a lifetime that is as short as possible.
        // The two references are then coerced to that lifetime.
        fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
            first * second
        }
    
        // `<'a: 'b, 'b>` reads as lifetime `'a` is at least as long as `'b`.
        // Here, we take in an `&'a i32` and return a `&'b i32` as a result of coercion.
        fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
            first
        }
        let first = 2; // Longer lifetime
        
        {
            let second = 3; // Shorter lifetime
            
            println!("The product is {}", multiply(&first, &second));
            println!("The product is {}", multiply(&second, &first));
            println!("{} is the first", choose_first(&first, &second));
            println!("{} is the first", choose_first(&second, &first)); // how did this work ???
        };
    }

    pub fn statics(){
        println!("\nLIFETIMES - STATICS:");
        // A reference with 'static lifetime:
        let _s: &'static str = "hello world";

        // 'static as part of a trait bound:
        #[allow(dead_code)]
        fn generic<T>(_x: T) where T: 'static {}

        // Make a constant with `'static` lifetime.
        static NUM: i32 = 18;

        // Returns a reference to `NUM` where its `'static`
        // lifetime is coerced to that of the input argument.
        fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
            &NUM
        }

        {
            // Make a `string` literal and print it:
            let static_string = "I'm in read-only memory";
            println!("static_string: {}", static_string);
    
            // When `static_string` goes out of scope, the reference
            // can no longer be used, but the data remains in the binary.
        }
    
        {
            // Make an integer to use for `coerce_static`:
            let lifetime_num = 9;
    
            // Coerce `NUM` to lifetime of `lifetime_num`:
            let coerced_static = coerce_static(&lifetime_num);
    
            println!("coerced_static: {}", coerced_static);
        }
    
        println!("NUM: {} stays accessible!", NUM);

        use std::fmt::Debug;

        fn print_it( input: impl Debug + 'static ) {
            println!( "'static value passed in is: {:?}", input );
        }

        fn print_it2( input: impl Debug) {
            println!( "'value passed in is: {:?}", input );
        }

        // i is owned and contains no references, thus it's 'static:
        let i = 5;
        print_it(i);

        // oops, &i only has the lifetime defined by the scope so it's not 'static:
        // print_it(&i);
        print_it2(&i);
    }

    pub fn ellision(){
        println!("\nLIFETIMES - ELLISION:");
        // `elided_input` and `annotated_input` essentially have identical signatures
        // because the lifetime of `elided_input` is inferred by the compiler:
        fn elided_input(x: &i32) {
            println!("`elided_input`: {}", x);
        }

        fn annotated_input<'a>(x: &'a i32) {
            println!("`annotated_input`: {}", x);
        }

        // Similarly, `elided_pass` and `annotated_pass` have identical signatures
        // because the lifetime is added implicitly to `elided_pass`:
        fn elided_pass(x: &i32) -> &i32 { x }

        fn annotated_pass<'a>(x: &'a i32) -> &'a i32 { x }

        let x = 3;

        elided_input(&x);
        annotated_input(&x);

        println!("`elided_pass`: {}", elided_pass(&x));
        println!("`annotated_pass`: {}", annotated_pass(&x));
    }
}






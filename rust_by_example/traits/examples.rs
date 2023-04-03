fn main() {
    basics::impl_trait_for_struct();
    auto_derive::comparison_traits();
    box_dyn::return_pointer_to_trait_on_heap();
    operator_overloading::add_foo_plus_bar();
    drop_trait::drop_it();
    iterators::impl_iterator_for_fibonacci();
    impl_traits::return_impl_trait();
    clone_trait::clone();
    trait_disambiguation::qualify();
}

mod basics {
    struct Sheep { naked: bool, name: &'static str }

    trait Animal {
        // Associated function signature; `Self` refers to the implementor type.
        fn new(name: &'static str) -> Self;

        // Method signatures; these will return a string.
        fn name(&self) -> &'static str;
        fn noise(&self) -> &'static str;

        // Traits can provide default method definitions.
        fn talk(&self) {
            println!("{} says {}", self.name(), self.noise());
        }
    }

    impl Sheep {
        fn is_naked(&self) -> bool {
            self.naked
        }

        fn shear(&mut self) {
            if self.is_naked() {
                // Implementor methods can use the implementor's trait methods.
                println!("{} is already naked...", self.name());
            } else {
                println!("{} gets a haircut!", self.name);

                self.naked = true;
            }
        }
    }

    // Implement the `Animal` trait for `Sheep`.
    impl Animal for Sheep {
        // `Self` is the implementor type: `Sheep`.
        fn new(name: &'static str) -> Sheep {
            Sheep { name: name, naked: false }
        }

        fn name(&self) -> &'static str {
            self.name
        }

        fn noise(&self) -> &'static str {
            if self.is_naked() {
                "baaaaah?"
            } else {
                "baaaaah!"
            }
        }
        
        // Default trait methods can be overridden.
        fn talk(&self) {
            // For example, we can add some quiet contemplation.
            println!("{} pauses briefly... {}", self.name, self.noise());
        }
    }

    pub fn impl_trait_for_struct() {
        println!("\nIMPL TRAIT FOR STRUCT:");
        // Type annotation is necessary in this case.
        let mut dolly: Sheep = Animal::new("Dolly");
        // TODO ^ Try removing the type annotations.

        dolly.talk();
        dolly.shear();
        dolly.shear();
        dolly.talk();
    }
}

mod auto_derive{
    // `Centimeters`, a tuple struct that can be compared
    #[derive(PartialEq, PartialOrd)]
    struct Centimeters(f64);

    // `Inches`, a tuple struct that can be printed
    #[derive(Debug)]
    struct Inches(i32);

    impl Inches {
        fn to_centimeters(&self) -> Centimeters {
            let &Inches(inches) = self;

            Centimeters(inches as f64 * 2.54)
        }
    }

    // `Seconds`, a tuple struct with no additional attributes
    struct Seconds(i32);

    pub fn comparison_traits() {
        println!("\nAUTO DERIVE COMPARISON TRAITS:");

        let _one_second = Seconds(1);

        // Error: `Seconds` can't be printed; it doesn't implement the `Debug` trait
        //println!("One second looks like: {:?}", _one_second);
        // TODO ^ Try uncommenting this line

        // Error: `Seconds` can't be compared; it doesn't implement the `PartialEq` trait
        //let _this_is_true = (_one_second == _one_second);
        // TODO ^ Try uncommenting this line

        let foot = Inches(12);

        println!("One foot equals {:?}", foot);

        let meter = Centimeters(100.0);

        let cmp =
            if foot.to_centimeters() < meter {
                "smaller"
            } else {
                "bigger"
            };

        println!("One foot is {} than one meter.", cmp);
    }
}

mod box_dyn {
    struct Sheep {}
    struct Cow {}

    trait Animal {
        // Instance method signature
        fn noise(&self) -> &'static str;
    }

    // Implement the `Animal` trait for `Sheep`.
    impl Animal for Sheep {
        fn noise(&self) -> &'static str {
            "baaaaah!"
        }
    }

    // Implement the `Animal` trait for `Cow`.
    impl Animal for Cow {
        fn noise(&self) -> &'static str {
            "moooooo!"
        }
    }

    // Returns some struct that implements Animal, but we don't know which one at compile time.
    fn random_animal(random_number: f64) -> Box<dyn Animal> {
        if random_number < 0.5 {
            Box::new(Sheep {})
        } else {
            Box::new(Cow {})
        }
    }

    pub fn return_pointer_to_trait_on_heap(){
        println!("\nBOX<DYN T> - RETURN POINTER TO TRAIT ON HEAP:");
        let random_number = 0.234;
        let animal = random_animal(random_number);
        println!("You've randomly chosen an animal, and it says {}", animal.noise());
    }

}

mod operator_overloading {
    use std::ops;

    struct Foo;
    struct Bar;

    #[derive(Debug)]
    struct FooBar;

    #[derive(Debug)]
    struct BarFoo;

    // The `std::ops::Add` trait is used to specify the functionality of `+`.
    // Here, we make `Add<Bar>` - the trait for addition with a RHS of type `Bar`.
    // The following block implements the operation: Foo + Bar = FooBar
    impl ops::Add<Bar> for Foo {
        type Output = FooBar;

        fn add(self, _rhs: Bar) -> FooBar {
            println!("> Foo.add(Bar) was called");

            FooBar
        }
    }

    // By reversing the types, we end up implementing non-commutative addition.
    // Here, we make `Add<Foo>` - the trait for addition with a RHS of type `Foo`.
    // This block implements the operation: Bar + Foo = BarFoo
    impl ops::Add<Foo> for Bar {
        type Output = BarFoo;

        fn add(self, _rhs: Foo) -> BarFoo {
            println!("> Bar.add(Foo) was called");

            BarFoo
        }
    }

    pub fn add_foo_plus_bar() {
        println!("\nOPERATOR OVERLOADING - ADD FOO + BAR:");
        println!("Foo + Bar = {:?}", Foo + Bar);
        println!("Bar + Foo = {:?}", Bar + Foo);
    }
}

mod drop_trait {
    struct Droppable {
        name: &'static str,
    }
    
    // This trivial implementation of `drop` adds a print to console.
    impl Drop for Droppable {
        fn drop(&mut self) {
            println!("> Dropping {}", self.name);
        }
    }
    
    pub fn drop_it() {
        println!("\nDROP TRAIT:");
        let _a = Droppable { name: "a" };
    
        // block A
        {
            let _b = Droppable { name: "b" };
    
            // block B
            {
                let _c = Droppable { name: "c" };
                let _d = Droppable { name: "d" };
    
                println!("Exiting block B");
            }
            println!("Just exited block B");
    
            println!("Exiting block A");
        }
        println!("Just exited block A");
    
        // Variable can be manually dropped using the `drop` function
        println!("about to manually drop a");
        drop(_a);
        // TODO ^ Try commenting this line
    
        println!("end of the main function");
    
        // `_a` *won't* be `drop`ed again here, because it already has been
        // (manually) `drop`ed
    }
}

mod iterators{
    struct Fibonacci {
        curr: u32,
        next: u32,
    }
    
    // Implement `Iterator` for `Fibonacci`.
    // The `Iterator` trait only requires a method to be defined for the `next` element.
    impl Iterator for Fibonacci {
        // We can refer to this type using Self::Item
        type Item = u32;
    
        // Here, we define the sequence using `.curr` and `.next`.
        // The return type is `Option<T>`:
        //     * When the `Iterator` is finished, `None` is returned.
        //     * Otherwise, the next value is wrapped in `Some` and returned.
        // We use Self::Item in the return type, so we can change
        // the type without having to update the function signatures.
        fn next(&mut self) -> Option<Self::Item> {
            let current = self.curr;
    
            self.curr = self.next;
            self.next = current + self.next;
    
            // Since there's no endpoint to a Fibonacci sequence, the `Iterator` 
            // will never return `None`, and `Some` is always returned.
            Some(current)
        }
    }
    
    // Returns a Fibonacci sequence generator
    fn fibonacci() -> Fibonacci {
        Fibonacci { curr: 0, next: 1 }
    }
    
    pub fn impl_iterator_for_fibonacci() {
        println!("\nITERATORS - IMPL ITERATOR FOR FINONACCI:");
        // `0..3` is an `Iterator` that generates: 0, 1, and 2.
        let mut sequence = 0..3;
    
        println!("FIVE consecutive `next` calls on 0..3");
        println!("> {:?}", sequence.next());
        println!("> {:?}", sequence.next());
        println!("> {:?}", sequence.next());
        println!("> {:?}", sequence.next());
        println!("> {:?}", sequence.next());
    
        // `for` works through an `Iterator` until it returns `None`.
        // Each `Some` value is unwrapped and bound to a variable (here, `i`).
        println!("Iterate through 0..3 using `for`");
        for i in 0..3 {
            println!("> {}", i);
        }
    
        // The `take(n)` method reduces an `Iterator` to its first `n` terms.
        println!("The first four terms of the Fibonacci sequence are: ");
        for i in fibonacci().take(4) {
            println!("> {}", i);
        }
    
        // The `skip(n)` method shortens an `Iterator` by dropping its first `n` terms.
        println!("The next four terms of the Fibonacci sequence are: ");
        for i in fibonacci().skip(4).take(4) {
            println!("> {}", i);
        }
    
        let array = [1u32, 3, 3, 7];
    
        // The `iter` method produces an `Iterator` over an array/slice.
        println!("Iterate the following array {:?}", &array);
        for i in array.iter() {
            println!("> {}", i);
        }
    }
}

mod impl_traits {
    // as generic function
    #[allow(dead_code)]
    fn parse_csv_document1<R: std::io::BufRead>(src: R) -> std::io::Result<Vec<Vec<String>>> {
        src.lines()
            .map(|line| {
                // For each line in the source
                line.map(|line| {
                    // If the line was read successfully, process it, if not, return the error
                    line.split(',') // Split the line separated by commas
                        .map(|entry| String::from(entry.trim())) // Remove leading and trailing whitespace
                        .collect() // Collect all strings in a row into a Vec<String>
                })
            })
            .collect() // Collect all lines into a Vec<Vec<String>>
    }
    // as impl train function
    #[allow(dead_code)]
    fn parse_csv_document2(src: impl std::io::BufRead) -> std::io::Result<Vec<Vec<String>>> {
        src.lines()
            .map(|line| {
                // For each line in the source
                line.map(|line| {
                    // If the line was read successfully, process it, if not, return the error
                    line.split(',') // Split the line separated by commas
                        .map(|entry| String::from(entry.trim())) // Remove leading and trailing whitespace
                        .collect() // Collect all strings in a row into a Vec<String>
                })
            })
            .collect() // Collect all lines into a Vec<Vec<String>>
    }

    use std::iter;
    use std::vec::IntoIter;

    // This function combines two `Vec<i32>` and returns an iterator over it.
    // Look how complicated its return type is!
    #[allow(dead_code)]
    fn combine_vecs_explicit_return_type(
        v: Vec<i32>,
        u: Vec<i32>,
    ) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
        v.into_iter().chain(u.into_iter()).cycle()
    }

    // This is the exact same function, but its return type uses `impl Trait`.
    // Look how much simpler it is!
    fn combine_vecs(
        v: Vec<i32>,
        u: Vec<i32>,
    ) -> impl Iterator<Item=i32> {
        v.into_iter().chain(u.into_iter()).cycle()
    }

    // Returns a function that adds `y` to its input
    fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
        let closure = move |x: i32| { x + y }; //note: `move` is required so that it wont outlive borrowed value `y`
        closure
    }

    fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
        numbers
            .iter()
            .filter(|x| x > &&0)
            .map(|x| x * 2)
    }

    pub fn return_impl_trait() {
        println!("\nRETURN IMPL TRAIT (SIMPLER SIGNATURE)");
        let v1 = vec![1, 2, 3];
        let v2 = vec![4, 5];
        let mut v3 = combine_vecs(v1, v2);
        assert_eq!(Some(1), v3.next());
        assert_eq!(Some(2), v3.next());
        assert_eq!(Some(3), v3.next());
        assert_eq!(Some(4), v3.next());
        assert_eq!(Some(5), v3.next());
        println!("all done");

        let plus_one = make_adder_function(1);
        assert_eq!(plus_one(2), 3);

        let singles = vec![-3, -2, 2, 3];
        let doubles = double_positives(&singles);
        assert_eq!(doubles.collect::<Vec<i32>>(), vec![4, 6]);
    }
}

mod clone_trait {
    // A unit struct without resources
    #[derive(Debug, Clone, Copy)] // Copy requires Clone
    struct Unit;

    // A tuple struct with resources that implements the `Clone` trait
    #[derive(Clone, Debug)]
    struct Pair(Box<i32>, Box<i32>);

    pub fn clone() {
        println!("\nCLONE TRAIT:");
        // Instantiate `Unit`
        let unit = Unit;
        // Copy `Unit`, there are no resources to move
        let copied_unit = unit; // , Copy trait - instead of taking ownership (move), makes a copy

        // Both `Unit`s can be used independently
        println!("original: {:?}", unit);
        println!("copy: {:?}", copied_unit);

        // Instantiate `Pair`
        let pair = Pair(Box::new(1), Box::new(2));
        println!("original: {:?}", pair);

        // Move `pair` into `moved_pair`, moves resources
        let moved_pair = pair;
        println!("moved: {:?}", moved_pair);

        // Error! `pair` has lost its resources
        //println!("original: {:?}", pair);
        // TODO ^ Try uncommenting this line

        // Clone `moved_pair` into `cloned_pair` (resources are included)
        let cloned_pair = moved_pair.clone();
        // Drop the original pair using std::mem::drop
        drop(moved_pair);

        // Error! `moved_pair` has been dropped
        //println!("copy: {:?}", moved_pair);
        // TODO ^ Try uncommenting this line

        // The result from .clone() can still be used!
        println!("clone: {:?}", cloned_pair);
    }
}

mod supertraits {
    trait Person {
        fn name(&self) -> String;
    }
    
    // Person is a supertrait of Student.
    // Implementing Student requires you to also impl Person.
    trait Student: Person {
        fn university(&self) -> String;
    }
    
    trait Programmer {
        fn fav_language(&self) -> String;
    }
    
    // CompSciStudent (computer science student) is a subtrait of both Programmer 
    // and Student. Implementing CompSciStudent requires you to impl both supertraits.
    trait CompSciStudent: Programmer + Student {
        fn git_username(&self) -> String;
    }
    
    #[allow(dead_code)]
    fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
        format!(
            "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
            student.name(),
            student.university(),
            student.fav_language(),
            student.git_username()
        )
    }
}

mod trait_disambiguation{
    trait UsernameWidget {
        // Get the selected username out of this widget
        fn get(&self) -> String;
    }
    
    trait AgeWidget {
        // Get the selected age out of this widget
        fn get(&self) -> u8;
    }
    
    // A form with both a UsernameWidget and an AgeWidget
    struct Form {
        username: String,
        age: u8,
    }
    
    impl UsernameWidget for Form {
        fn get(&self) -> String {
            self.username.clone()
        }
    }
    
    impl AgeWidget for Form {
        fn get(&self) -> u8 {
            self.age
        }
    }
    
    pub fn qualify() {
        println!("\nDISAMBIGUATE TRAITS:");
        let form = Form {
            username: "rustacean".to_owned(),
            age: 28,
        };
    
        // If you uncomment this line, you'll get an error saying
        // "multiple `get` found". Because, after all, there are multiple methods
        // named `get`.
        // println!("{}", form.get());
    
        let username = <Form as UsernameWidget>::get(&form);
        assert_eq!("rustacean".to_owned(), username);
        let age = <Form as AgeWidget>::get(&form);
        assert_eq!(28, age);
    }
}

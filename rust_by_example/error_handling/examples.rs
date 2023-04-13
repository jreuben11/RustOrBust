#[cfg(not(feature = "foo"))]
fn main() {
    dont_panic("ok");
    dont_panic("abort");
    // dont_panic("");
    conditional_compile();// rustc  examples.rs -C panic=unwind 

    option_unwrapping::unwrap_none_panic();
    option_unwrapping::option_chaining(); // ?
    option_unwrapping::map_combinators();  // map(|Option<X>| Option<Y>)
    option_unwrapping::flatmap_combinators(); // and_then()
    option_unwrapping::eager_chainable(); // or()
    option_unwrapping::lazy_chainable(); // or_else()
    option_unwrapping::eager_inplace(); // get_or_insert()
    option_unwrapping::lazy_inplace(); // get_or_insert_with()

    results::combinators(); // map(), and_then()
    results::aliased_result_type();
    results::early_returns();
    results::early_returns_succint(); // ? 

    result_multiple_error_types::results_from_options(); // parse::<i32>().map(|i| 2 * i).map_err(|_| DoubleError)
    result_multiple_error_types::custom_error_types();
    result_multiple_error_types::box_errors(); // Option<T>::into

    wrapping_errors::call();

    iterating_over_results::iter_map();
    iterating_over_results::ignore_errors();
    iterating_over_results::collect_errors();
    iterating_over_results::result_from_iterator();
    iterating_over_results::partition();

}

#[allow(unused_imports)]
use std::num::ParseIntError;
// --cfg feature="${feature_name}"
// rustc  examples.rs --cfg 'feature="foo"'
#[cfg(feature = "foo")]
fn main() -> Result<(), ParseIntError> {
    let number_str = "10";
    let number = match number_str.parse::<i32>() {
        Ok(number)  => number,
        Err(e) => return Err(e),
    };
    println!("{}", number);
    Ok(())
}

fn dont_panic(input: &str) {
    // You shouldn't drink too much sugary beverages.
    if input.is_empty() { panic!("AAAaaaaa!!!!"); }
    else if input == "abort" && cfg!(panic="abort"){  // rustc  examples.rs -C panic=abort
        println!("abort");
    }
    else {
        println!("input {} is all you need.", input);
    } 
}

// codegen option `panic` - either `unwind` or `abort` is expected
#[cfg(panic = "unwind")]
fn conditional_compile(){ println!("compiler flag");}

#[cfg(not(panic="unwind"))]
fn conditional_compile(){ println!("no compiler flag");}

mod option_unwrapping {
    // The adult has seen it all, and can handle any drink well.
    // All drinks are handled explicitly using `match`.
    fn give_adult(drink: Option<&str>) {
        // Specify a course of action for each case.
        match drink {
            Some("lemonade") => println!("Yuck! Too sugary."),
            Some(inner)   => println!("{}? How nice.", inner),
            None          => println!("No drink? Oh well."),
        }
    }

    // Others will `panic` before drinking sugary drinks.
    // All drinks are handled implicitly using `unwrap`.
    fn drink(drink: Option<&str>) {
        // `unwrap` returns a `panic` when it receives a `None`.
        let inside = drink.unwrap();
        if inside == "lemonade" { panic!("AAAaaaaa!!!!"); }

        println!("I love {}s!!!!!", inside);
    }

    pub fn unwrap_none_panic() {
        println!("\nOPTION UNWRAPPING - UNWRAP NONE PANIC:");
        let water  = Some("water");
        let lemonade = Some("lemonade");
        // let void  = None;

        give_adult(water);
        give_adult(lemonade);
        // give_adult(void);

        let coffee = Some("coffee");
        // let nothing = None;

        drink(coffee);
        // drink(nothing);
    }

    fn next_birthday(current_age: Option<u8>) -> Option<String> {
        // If `current_age` is `None`, this returns `None`.
        // If `current_age` is `Some`, the inner `u8` gets assigned to `next_age`
        let next_age: u8 = current_age? + 1;
        Some(format!("Next year I will be {}", next_age))
    }

    #[derive(Debug)]
    struct Person {
        job: Option<Job>,
        age: Option<u8>,
    }
    
    #[derive(Debug, Clone, Copy)]
    struct Job {
        phone_number: Option<PhoneNumber>,
    }
    
    #[derive(Debug, Clone, Copy)]
    struct PhoneNumber {
        area_code: Option<u8>,
        number: u32,
    }
    
    impl Person {
    
        // Gets the area code of the phone number of the person's job, if it exists.
        fn work_phone_area_code(&self) -> Option<u8> {
            // This would need many nested `match` statements without the `?` operator.
            // It would take a lot more code - try writing it yourself and see which
            // is easier.
            self.job?.phone_number?.area_code
        }
    }
    
    pub fn option_chaining() {
        println!("\nOPTION UNWRAPPING - CHAINING WITH ?:");
        let p = Person {
            job: Some(Job {
                phone_number: Some(PhoneNumber {
                    area_code: Some(61),
                    number: 439222222,
                }),
            }),
            age: Some(10u8),
        };
        
        println!("{:?}",p); // requires Debug traits

        // cannot use the `?` operator in a function that returns `()`:
        // let x = p.job?.phone_number;
        
        // ugly unwraps can panic !!!! better option is to use nested match. even better use nested ? with let Some
        println!("area code: {}", p.job.unwrap().phone_number.unwrap().area_code.unwrap());
        println!("number: {}", p.job.unwrap().phone_number.unwrap().number);
        println!("{}", next_birthday(p.age).unwrap());

        // uses ? internally
        if let Some(i) = p.work_phone_area_code() {
            println!("area code: {}",i);
        }
    }


    #[derive(Debug)] enum Food { Apple, Carrot, _Potato }
    #[derive(Debug)] struct Peeled(Food);
    #[derive(Debug)] struct Chopped(Food);
    #[derive(Debug)] struct Cooked(Food);

    // Peeling food. If there isn't any, then return `None`.
    // Otherwise, return the peeled food.
    fn peel(food: Option<Food>) -> Option<Peeled> {
        match food {
            Some(food) => Some(Peeled(food)),
            None       => None,
        }
    }

    // Chopping food. If there isn't any, then return `None`.
    // Otherwise, return the chopped food.
    fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
        match peeled {
            Some(Peeled(food)) => Some(Chopped(food)),
            None               => None,
        }
    }

    // Cooking food. Here, we showcase `map()` instead of `match` for case handling.
    fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
        chopped.map(|Chopped(food)| Cooked(food))
    }

    // A function to peel, chop, and cook food all in sequence.
    // We chain multiple uses of `map()` to simplify the code.
    fn process(food: Option<Food>) -> Option<Cooked> {
        food.map(|f| Peeled(f))
            .map(|Peeled(f)| Chopped(f))
            .map(|Chopped(f)| Cooked(f))
    }

    // Check whether there's food or not before trying to eat it!
    fn eat(food: Option<Cooked>) {
        match food {
            Some(food) => println!("Mmm. I love {:?}", food),
            None       => println!("Oh no! It wasn't edible."),
        }
    }

    pub fn map_combinators() {
        println!("\nOPTION UNWRAPPING - MAP COMBINATORS:");
        let apple = Some(Food::Apple);
        let carrot = Some(Food::Carrot);
        let potato = None;

        let cooked_apple = cook(chop(peel(apple)));
        let cooked_carrot = cook(chop(peel(carrot)));
        // Let's try the simpler looking `process()` now.
        let cooked_potato = process(potato);

        eat(cooked_apple);
        eat(cooked_carrot);
        eat(cooked_potato);
    }



    #[derive(Debug)] enum FancyFood { CordonBleu, Steak, Sushi }
    #[derive(Debug)] enum Day { Monday, Tuesday, Wednesday }
    
    // We don't have the ingredients to make Sushi.
    fn have_ingredients(food: FancyFood) -> Option<FancyFood> {
        match food {
            FancyFood::Sushi => None,
            _           => Some(food),
        }
    }
    
    // We have the recipe for everything except Cordon Bleu.
    fn have_recipe(food: FancyFood) -> Option<FancyFood> {
        match food {
            FancyFood::CordonBleu => None,
            _                => Some(food),
        }
    }
    
    // To make a dish, we need both the recipe and the ingredients.
    // We can represent the logic with a chain of `match`es:
    #[allow(dead_code)]
    fn cookable_v1(food: FancyFood) -> Option<FancyFood> {
        match have_recipe(food) {
            None       => None,
            Some(food) => match have_ingredients(food) {
                None       => None,
                Some(food) => Some(food),
            },
        }
    }
    
    // This can conveniently be rewritten more compactly with `and_then()`:
    fn cookable_v2(food: FancyFood) -> Option<FancyFood> {
        have_recipe(food).and_then(have_ingredients)
    }
    
    fn fancy_eat(food: FancyFood, day: Day) {
        match cookable_v2(food) {
            Some(food) => println!("Yay! On {:?} we get to eat {:?}.", day, food),
            None       => println!("Oh no. We don't get to eat on {:?}?", day),
        }
    }
    
    
    pub fn flatmap_combinators() {
        println!("\nOPTION UNWRAPPING - FLATMAP COMBINATORS:");
        let (cordon_bleu, steak, sushi) = (FancyFood::CordonBleu, FancyFood::Steak, FancyFood::Sushi);
    
        fancy_eat(cordon_bleu, Day::Monday);
        fancy_eat(steak, Day::Tuesday);
        fancy_eat(sushi, Day::Wednesday);
    }

    #[derive(Debug)] 
    enum Fruit { Apple, Orange, Banana, Kiwi, Lemon }

    pub fn eager_chainable() {
        println!("\nOPTION UNWRAPPING - EAGER CHAINABLE (or):");
        let _apple = Some(Fruit::Apple);
        let orange = Some(Fruit::Orange);
        let no_fruit: Option<Fruit> = None;

        let first_available_fruit = no_fruit.or(orange).or(_apple);
        println!("first_available_fruit: {:?}", first_available_fruit);
        // first_available_fruit: Some(Orange)

        // `or` moves its argument.
        // In the example above, `or(orange)` returned a `Some`, so `or(apple)` was not invoked.
        // But the variable named `apple` has been moved regardless, and cannot be used anymore.
        // println!("Variable apple was moved, so this line won't compile: {:?}", _apple);
        // TODO: uncomment the line above to see the compiler error
    }

    pub fn lazy_chainable() {
        println!("\nOPTION UNWRAPPING - LAZY CHAINABLE (or_else):");
        let no_fruit: Option<Fruit> = None;
        let get_kiwi_as_fallback = || {
            println!("Providing kiwi as fallback");
            Some(Fruit::Kiwi)
        };
        let get_lemon_as_fallback = || {
            println!("Providing lemon as fallback");
            Some(Fruit::Lemon)
        };
    
        let first_available_fruit = no_fruit
            .or_else(get_kiwi_as_fallback)
            .or_else(get_lemon_as_fallback);
        println!("first_available_fruit: {:?}", first_available_fruit);
        // Providing kiwi as fallback
        // first_available_fruit: Some(Kiwi)
    }

    pub fn eager_inplace() {
        println!("\nOPTION UNWRAPPING - EAGER INPLACE (get_or_insert):");
        let mut my_fruit: Option<Fruit> = None;
        let banana = Fruit::Banana;
        let first_available_fruit = my_fruit.get_or_insert(banana);
        println!("first_available_fruit is: {:?}", first_available_fruit);
        println!("my_fruit is: {:?}", my_fruit); // immutable borrow
        // -> error - cannot borrow `my_fruit` as immutable because it is also borrowed as mutable here:
        // println!("first_available_fruit is: {:?}", first_available_fruit);
        // TODO: uncomment the line above to see the compiler error

        // first_available_fruit is: Banana
        // my_fruit is: Some(Banana)
        //println!("Variable named `banana` is moved: {:?}", banana);
        // TODO: uncomment the line above to see the compiler error
    }

    pub fn lazy_inplace() {
        println!("\nOPTION UNWRAPPING - LAZY INPLACE (get_or_insert_with):");
        let mut my_fruit: Option<Fruit> = None;
        let get_lemon_as_fallback = || {
            println!("Providing lemon as fallback");
            Fruit::Lemon
        };
        let first_available_fruit = my_fruit
            .get_or_insert_with(get_lemon_as_fallback);
        println!("first_available_fruit is: {:?}", first_available_fruit);
        println!("my_fruit is: {:?}", my_fruit);
        // Providing lemon as fallback
        // first_available_fruit is: Lemon
        // my_fruit is: Some(Lemon)
    
        // If the Option has a value, it is left unchanged, and the closure is not invoked
        let mut my_apple = Some(Fruit::Apple);
        let should_be_apple = my_apple.get_or_insert_with(get_lemon_as_fallback);
        println!("should_be_apple is: {:?}", should_be_apple);
        println!("my_apple is unchanged: {:?}", my_apple);
        // The output is a follows. Note that the closure `get_lemon_as_fallback` is not invoked
        // should_be_apple is: Apple
        // my_apple is unchanged: Some(Apple)
    }
}

mod results {
    use std::num::ParseIntError;

    // As with `Option`, we can use combinators such as `map()`.
    // This function is otherwise identical to the one above and reads:
    // Modify n if the value is valid, otherwise pass on the error.
    fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
        first_number_str.parse::<i32>().and_then(|first_number| {
            second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
        })
    }

    fn print(result: Result<i32, ParseIntError>) {
        match result {
            Ok(n)  => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    pub fn combinators() {
        println!("\nRESULTS - COMBINATORS (map, and_then):");
        // This still presents a reasonable answer.
        let twenty = multiply("10", "2");
        print(twenty);

        // The following now provides a much more helpful error message.
        let tt = multiply("t", "2");
        print(tt);
    }

    // Define a generic alias for a `Result` with the error type `ParseIntError`.
    type AliasedResult<T> = Result<T, ParseIntError>;
    // Use the above alias to refer to our specific `Result` type.
    fn multiply2(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
        first_number_str.parse::<i32>().and_then(|first_number| {
            second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
        })
    }

    // Here, the alias again allows us to save some space.
    fn print2(result: AliasedResult<i32>) {
        match result {
            Ok(n)  => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    pub fn aliased_result_type() {
        println!("\nRESULTS - ALIASED RESULT TYPE:");
        print2(multiply2("10", "2"));
        print2(multiply2("t", "2"));
    }


    fn multiply3(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
        let first_number = match first_number_str.parse::<i32>() {
            Ok(first_number)  => first_number,
            Err(e) => return Err(e),
        };

        let second_number = match second_number_str.parse::<i32>() {
            Ok(second_number)  => second_number,
            Err(e) => return Err(e),
        };

        Ok(first_number * second_number)
    }

    pub fn early_returns() {
        println!("\nRESULTS - EARLY RETURNS:");
        print2(multiply3("10", "2"));
        print2(multiply3("t", "2"));
    }

    fn multiply4(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
        let first_number = first_number_str.parse::<i32>()?;
        let second_number = second_number_str.parse::<i32>()?;
    
        Ok(first_number * second_number)
    }

    pub fn early_returns_succint() {
        println!("\nRESULTS - EARLY RETURNS USING ?:");
        print2(multiply4("10", "2"));
        print2(multiply4("t", "2"));
    }
}

mod result_multiple_error_types {
    use std::num::ParseIntError;

    fn double_first(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
        let opt = vec.first().map(|first| {
            first.parse::<i32>().map(|n| 2 * n)
        });
    
        opt.map_or(Ok(None), |r| r.map(Some))
    }
    
    pub fn results_from_options() {
        println!("\nRESULT MULTIPLE ERROR TYPES - RESULTS FROM OPTIONS:");
        let numbers = vec!["42", "93", "18"];
        let empty = vec![];
        let strings = vec!["tofu", "93", "18"];
    
        println!("The first doubled is {:?}", double_first(numbers));
        println!("The first doubled is {:?}", double_first(empty));
        println!("The first doubled is {:?}", double_first(strings));
    }

    use std::fmt;

    type MyResult<T> = std::result::Result<T, DoubleError>;

    // Define our error types. These may be customized for our error handling cases.
    // Now we will be able to write our own errors, defer to an underlying error
    // implementation, or do something in between.
    #[derive(Debug, Clone)]
    struct DoubleError;

    // Generation of an error is completely separate from how it is displayed.
    // There's no need to be concerned about cluttering complex logic with the display style.
    //
    // Note that we don't store any extra info about the errors. This means we can't state
    // which string failed to parse without modifying our types to carry that information.
    impl fmt::Display for DoubleError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "invalid first item to double")
        }
    }

    fn double_first2(vec: Vec<&str>) -> MyResult<i32> {
        vec.first()
            // Change the error to our new type.
            .ok_or(DoubleError)
            .and_then(|s| {
                s.parse::<i32>()
                    // Update to the new error type here also.
                    .map(|i| 2 * i)
                    .map_err(|_| DoubleError)
            })
    }

    fn print(result: MyResult<i32>) {
        match result {
            Ok(n) => println!("The first doubled is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    pub fn custom_error_types() {
        println!("\nRESULT MULTIPLE ERROR TYPES - CUSTOM ERROR TYPES:");
        let numbers = vec!["42", "93", "18"];
        let empty = vec![];
        let strings = vec!["tofu", "93", "18"];

        print(double_first2(numbers));
        print(double_first2(empty));
        print(double_first2(strings));
    }

    use std::error;

    // Change the alias to `Box<error::Error>`.
    type BoxErrorResult<T> = std::result::Result<T, Box<dyn error::Error>>;

    #[derive(Debug, Clone)]
    struct EmptyVec;

    impl fmt::Display for EmptyVec {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "invalid first item to double")
        }
    }

    impl error::Error for EmptyVec {}

    fn double_first3(vec: &Vec<&str>) -> BoxErrorResult<i32> {
        vec.first()
            .ok_or_else(|| EmptyVec.into()) // Converts to Box
            .and_then(|s| {
                s.parse::<i32>()
                    .map_err(|e| e.into()) // Converts to Box
                    .map(|i| 2 * i)
            })
    }

    fn print2(result: BoxErrorResult<i32>) {
        match result {
            Ok(n) => println!("The first doubled is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    // The same structure as before but rather than chain all `Results`
    // and `Options` along, we `?` to get the inner value out immediately.
    fn double_first4(vec: &Vec<&str>) -> BoxErrorResult<i32> {
        let first = vec.first().ok_or(EmptyVec)?;
        let parsed = first.parse::<i32>()?;
        Ok(2 * parsed)
    }

    pub fn box_errors() {
        println!("\nRESULT MULTIPLE ERROR TYPES - BOX ERRORS (Option<T>::into):");
        let numbers = vec!["42", "93", "18"];
        let empty = vec![];
        let strings = vec!["tofu", "93", "18"];

        print2(double_first3(&numbers));
        print2(double_first3(&empty));
        print2(double_first3(&strings));

        print2(double_first4(&numbers));
        print2(double_first4(&empty));
        print2(double_first4(&strings));
    }
}

mod wrapping_errors {
    use std::error;
    use std::error::Error;
    use std::num::ParseIntError;
    use std::fmt;

    type Result<T> = std::result::Result<T, DoubleError>;

    #[derive(Debug)]
    enum DoubleError {
        EmptyVec,
        // We will defer to the parse error implementation for their error.
        // Supplying extra info requires adding more data to the type.
        Parse(ParseIntError),
    }

    impl fmt::Display for DoubleError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                DoubleError::EmptyVec =>
                    write!(f, "please use a vector with at least one element"),
                // The wrapped error contains additional information and is available
                // via the source() method.
                DoubleError::Parse(..) =>
                    write!(f, "the provided string could not be parsed as int"),
            }
        }
    }

    impl error::Error for DoubleError {
        fn source(&self) -> Option<&(dyn error::Error + 'static)> {
            match *self {
                DoubleError::EmptyVec => None,
                // The cause is the underlying implementation error type. Is implicitly
                // cast to the trait object `&error::Error`. This works because the
                // underlying type already implements the `Error` trait.
                DoubleError::Parse(ref e) => Some(e),
            }
        }
    }

    // Implement the conversion from `ParseIntError` to `DoubleError`.
    // This will be automatically called by `?` if a `ParseIntError`
    // needs to be converted into a `DoubleError`.
    impl From<ParseIntError> for DoubleError {
        fn from(err: ParseIntError) -> DoubleError {
            DoubleError::Parse(err)
        }
    }

    fn double_first(vec: Vec<&str>) -> Result<i32> {
        let first = vec.first().ok_or(DoubleError::EmptyVec)?;
        // Here we implicitly use the `ParseIntError` implementation of `From` (which
        // we defined above) in order to create a `DoubleError`.
        let parsed = first.parse::<i32>()?;

        Ok(2 * parsed)
    }

    fn print(result: Result<i32>) {
        match result {
            Ok(n)  => println!("The first doubled is {}", n),
            Err(e) => {
                println!("Error: {}", e);
                if let Some(source) = e.source() {
                    println!("  Caused by: {}", source);
                }
            },
        }
    }

    pub fn call() {
        println!("\nWRAPPING ERRORS:");
        let numbers = vec!["42", "93", "18"];
        let empty = vec![];
        let strings = vec!["tofu", "93", "18"];

        print(double_first(numbers));
        print(double_first(empty));
        print(double_first(strings));
    }
}

mod iterating_over_results {
    pub fn iter_map() {
        println!("\nITERATING OVER RESULTS - ITER MAP:");
        let strings = vec!["tofu", "93", "18"];
        let numbers: Vec<_> = strings
            .into_iter()
            .map(|s| s.parse::<i32>())
            .collect();
        println!("Results: {:?}", numbers);
    }

    pub fn ignore_errors() {
        println!("\nITERATING OVER RESULTS - IGNORE ERRORS:");
        let strings = vec!["tofu", "93", "18"];
        let numbers: Vec<_> = strings
            .into_iter()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        println!("Results: {:?}", numbers);
    }

    pub fn collect_errors() {
        println!("\nITERATING OVER RESULTS - COLLECT ERRORS TOO:");
        let strings = vec!["42", "tofu", "93", "999", "18"];
        let mut errors = vec![];
        let numbers: Vec<_> = strings
            .into_iter()
            .map(|s| s.parse::<u8>())
            .filter_map(|r| r.map_err(|e| errors.push(e)).ok())
            .collect();
        println!("Numbers: {:?}", numbers);
        println!("Errors: {:?}", errors);
    }

    pub fn result_from_iterator() {
        println!("\nITERATING OVER RESULTS - RESULT FROM ITERATOR (1st error):");
        // Result implements FromIterator so that a vector of results (Vec<Result<T, E>>) can be turned into a result with a vector (Result<Vec<T>, E>).
        let strings = vec!["tofu", "93", "18"];
        let numbers: Result<Vec<_>, _> = strings 
            .into_iter()
            .map(|s| s.parse::<i32>())
            .collect();
        println!("Results: {:?}", numbers);
    }

    pub fn partition() {
        println!("\nITERATING OVER RESULTS - PARTITION:");

        let strings = vec!["tofu", "93", "18"];
        let (numbers, errors): (Vec<_>, Vec<_>) = strings
            .into_iter()
            .map(|s| s.parse::<i32>())
            .partition(Result::is_ok);
        let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
        let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
        println!("Numbers: {:?}", numbers);
        println!("Errors: {:?}", errors);
    }
}


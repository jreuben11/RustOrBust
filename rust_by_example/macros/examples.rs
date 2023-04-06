// This is a simple macro named `say_hello`.
macro_rules! say_hello {
    // `()` indicates that the macro takes no argument.
    () => {
        // The macro will expand into the contents of this block.
        println!("Hello!")
    };
}

macro_rules! create_function {
    // This macro takes an argument of designator `ident` and
    // creates a function named `$func_name`.
    // The `ident` designator is used for variable/function names.
    ($func_name:ident) => {
        fn $func_name() {
            // The `stringify!` macro converts an `ident` into a string.
            println!("You called {:?}()",
                    stringify!($func_name));
        }
    };
}

macro_rules! print_result {
    // This macro takes an expression of type `expr` and prints
    // it as a string along with its result.
    // The `expr` designator is used for expressions.
    ($expression:expr) => {
        // `stringify!` will convert the expression *as it is* into a string.
        println!("{:?} = {:?}",
                stringify!($expression),
                $expression);
    };
}



// `test_overloading!` will compare `$left` and `$right`
// in different ways depending on how you invoke it:
macro_rules! test_overloading {
    // Arguments don't need to be separated by a comma.
    // Any template can be used!
    ($left:expr; and $right:expr) => {
        println!("{:?} and {:?} is {:?}",
                stringify!($left),
                stringify!($right),
                $left && $right)
    };
    // ^ each arm must end with a semicolon.
    ($left:expr; or $right:expr) => {
        println!("{:?} or {:?} is {:?}",
                stringify!($left),
                stringify!($right),
                $left || $right)
    };
}

// `find_min!` will calculate the minimum of any number of arguments.
macro_rules! find_min_recursively {
    // Base case:
    ($x:expr) => ($x);
    // `$x` followed by at least one `$y,`
    ($x:expr, $($y:expr),+) => (
        // Call `find_min!` on the tail `$y`
        std::cmp::min($x, find_min_recursively!($($y),+))
    )
}

use std::ops::{Add, Mul, Sub};

macro_rules! assert_equal_len {
    // The `tt` (token tree) designator is used for
    // operators and tokens.
    ($a:expr, $b:expr, $func:ident, $op:tt) => {
        assert!($a.len() == $b.len(),
                "{:?}: dimension mismatch: {:?} {:?} {:?}",
                stringify!($func),
                ($a.len(),),
                stringify!($op),
                ($b.len(),));
    };
}

macro_rules! op {
    ($func:ident, $bound:ident, $op:tt, $method:ident) => {
        fn $func<T: $bound<T, Output=T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
            assert_equal_len!(xs, ys, $func, $op);

            for (x, y) in xs.iter_mut().zip(ys.iter()) {
                *x = $bound::$method(*x, *y);
                // *x = x.$method(*y);
            }
        }
    };
}


macro_rules! test_op {
    ($func:ident, $x:expr, $y:expr, $z:expr) => {
        #[test]
        fn $func() {
            use std::iter;
            for size in 0usize..10 {
                let mut x: Vec<_> = iter::repeat($x).take(size).collect();
                let y: Vec<_> = iter::repeat($y).take(size).collect();
                let z: Vec<_> = iter::repeat($z).take(size).collect();

                super::$func(&mut x, &y);

                assert_eq!(x, z);
            }
        }
    };
}

macro_rules! calculate_dsl {
    (eval $e:expr) => {
        {
            let val: usize = $e; // Force types to be integers
            println!("{} = {}", stringify!{$e}, val);
        }
    };
}

macro_rules! calculate_variadic {
    // The pattern for a single `eval`
    (eval $e:expr) => {
        {
            let val: usize = $e; // Force types to be integers
            println!("{} = {}", stringify!{$e}, val);
        }
    };

    // Decompose multiple `eval`s recursively
    (eval $e:expr, $(eval $es:expr),+) => {{
        calculate_variadic! { eval $e }
        calculate_variadic! { $(eval $es),+ }
    }};
}


fn main() {
    // macros have to be declared before they are used in the .rs file

    // This call will expand into `println!("Hello");`
    say_hello!();
    
    // ident designator: Create functions named `foo` and `bar`
    create_function!(foo);
    create_function!(bar);
    foo();
    bar();

    print_result!(1u32 + 1);
    //  expr designator: blocks are expressions too
    print_result!({
        let x = 1u32;
        x * x + 2 * x - 1
    });

    test_overloading!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test_overloading!(true; or false);

    println!("{}", find_min_recursively!(1));
    println!("{}", find_min_recursively!(1 + 2, 2));
    println!("{}", find_min_recursively!(5, 2 * 3, 4));

    // Implement `add_assign`, `mul_assign`, and `sub_assign` functions.
    op!(add_assign, Add, +=, add);
    op!(mul_assign, Mul, *=, mul);
    op!(sub_assign, Sub, -=, sub);

    // Test `add_assign`, `mul_assign`, and `sub_assign`.
    test_op!(add_assign, 1u32, 2u32, 3u32);
    test_op!(mul_assign, 2u32, 3u32, 6u32);
    test_op!(sub_assign, 3u32, 2u32, 1u32);

    use std::iter;
    let mut x: Vec<_> = iter::repeat(1).take(10).collect();
    let y: Vec<_> = iter::repeat(2).take(10).collect();
    add_assign(&mut x, &y);
    println!("add_assign:{:?}", &x);
    mul_assign(&mut x, &y);
    println!("mulassign:{:?}", &x);
    sub_assign(&mut x, &y);
    println!("sub_assign:{:?}", &x);

    calculate_dsl! {
        eval 1 + 2 // hehehe `eval` is _not_ a Rust keyword!
    }
    calculate_dsl! {
        eval (1 + 2) * (3 / 4)
    }
    calculate_variadic! { 
        eval 1 + 2,
        eval 3 + 4,
        eval (2 * 3) + 1
    }
}











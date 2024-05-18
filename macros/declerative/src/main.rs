#![feature(trace_macros)]
#![feature(log_syntax)]

#[macro_use]
mod my_vec;

#[macro_use]
mod greeting;
use crate::greeting::base_greeting_fn;

#[macro_use]
mod generate_get_value;

use crate::account_dsl::{Account, Currency};
#[macro_use]
mod account_dsl;

#[macro_use]
mod recursive_compose;
use crate::recursive_compose::compose_two;

#[macro_use]
mod hello_world;

fn main() {
    custom_vec();
    variadic_greeting();
    recursive_newtype::create();
    use_account_dsl();
    compose_vector_of_fn();
    add_impl();
}

fn custom_vec() {
    let empty: Vec<i32> = my_vec![];
    println!("{:?}", empty);
    let also_empty: Vec<i32> = my_vec!(make an empty vec);
    println!("{:?}", also_empty);
    let three_numbers = my_vec!(1, 2, 3);
    println!("{:?}", three_numbers);
}

fn variadic_greeting() {
    // variadic, debug
    trace_macros!(true);
    let greet = greeting!("Sam", "Heya");
    let greet_with_default = greeting!("Sam");
    let _greet_with_default_test = greeting!(test "Sam");
    trace_macros!(false);
    println!("{}", greet);
    println!("{}", greet_with_default);
}

mod recursive_newtype {
    struct Age {
        value: i32,
    }
    struct Name {
        value: String,
    }
    generate_newtypes_methods!(Name);
    generate_get_value_string!(Age, i32);
    generate_from!(Age, i32);

    pub fn create() {
        let age = Age { value: 1 };
        let name = Name {
            value: "blah".to_string(),
        };
        

        println!("{}, {}", age.get_value(), name.get_value());
        let _x: i32 = age.into();
        let _y: String = name.into();
    }
}

fn use_account_dsl() {
    let mut the_poor = Account { money: 0 };
    let mut the_rich = Account { money: 200 };
    exchange!(Give 0 to the_poor);
    exchange!(Give 20 to the_poor);
    exchange!(Give 1 "euros" to the_poor);
    exchange!(Give 1 "euro" to the_poor);
    exchange!(Give 1 "dollar" to the_poor);
    exchange!(Take 10 from the_rich);
    exchange!(Give 30 from the_rich to the_poor);
    println!("Poor: {:?}, rich: {:?}", the_poor, the_rich);
}

fn compose_vector_of_fn() {
    fn add_one(n: i32) -> i32 {
        n + 1
    }
    fn stringify(n: i32) -> String {
        n.to_string()
    }
    fn prefix_with(prefix: &str) -> impl Fn(String) -> String + '_ {
        move |x| format!("{}{}", prefix, x)
    }

    let composed1 = compose!(add_one, stringify, prefix_with("Result: "));
    println!("{}", composed1(5));
    let composed2 = compose_alt!(
      add_one => stringify => prefix_with("Result: ")
    );
    println!("{}", composed2(5));
}

struct Example {}
hello_world!(Example); 
fn add_impl(){
    let e = Example {}; 
    e.hello_world();
}

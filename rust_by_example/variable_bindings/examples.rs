fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // copy `an_integer` into `copied_integer`
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    mutability();
    scope();
    variable_shadowing();
    declare_first();
    freezing();

}

fn mutability() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);
    // Ok
    mutable_binding += 1;
    println!("After mutation: {}", mutable_binding);

    // Error!
    // _immutable_binding += 1;
    // FIXME ^ Comment out this line
}

fn scope() {
    // This binding lives in the main function
    let long_lived_binding = 1;

    // This is a block, and has a smaller scope than the main function
    {
        // This binding only exists in this block
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);
    }
    // End of the block

    // Error! `short_lived_binding` doesn't exist in this scope
    // println!("outer short: {}", short_lived_binding);
    // FIXME ^ Comment out this line

    println!("outer long: {}", long_lived_binding);
}

fn variable_shadowing() {
    let shadowed_binding = 1;

    {
        println!("before being shadowed: {}", shadowed_binding);

        // This binding *shadows* the outer one
        let shadowed_binding = "abc";

        println!("shadowed in inner block: {}", shadowed_binding);
    }
    println!("outside inner block: {}", shadowed_binding);

    // This binding *shadows* the previous binding
    let shadowed_binding = 2;
    println!("shadowed in outer block: {}", shadowed_binding);
}

fn declare_first() {
    // Declare a variable binding
    let a_binding;
    {
        let x = 2;
        // Initialize the binding
        a_binding = x * x;
    }
    println!("a binding: {}", a_binding);
    let another_binding;
    // Error! Use of uninitialized binding
    // println!("another binding: {}", another_binding);
    // FIXME ^ Comment out this line
    another_binding = 1;
    println!("another binding: {}", another_binding);
}

fn freezing() {
    let mut _mutable_integer = 7i32;
    {
        // Shadowing by immutable `_mutable_integer`
        let _mutable_integer = _mutable_integer;
        // Error! `_mutable_integer` is frozen in this scope
        // _mutable_integer = 50;
        // FIXME ^ Comment out this line
        // `_mutable_integer` goes out of scope
    }

    // Ok! `_mutable_integer` is not frozen in this scope
    _mutable_integer = 3;
}
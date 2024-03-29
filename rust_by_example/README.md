# To run:
```bash
cd xxx
rustc examples.rs
./examples
```

# TOC
1. [hello_world](hello_world/examples.rs)
    ```rust
    formatted_print();
    print_debug();
    print_display();
    print_display2();
    composite_formatting();
    ```
2. [primatives](primatives/examples.rs)
    ```rust
    scalars();
    literals_and_operators();
    tuples();
    arrays_and_slices();
    ```

3. [custom types](custom_types/examples.rs)
   ```rust
    structs();
    enums::enum_match();
    enums::enum_use();
    enums::c_like_enums();
    enums::linked_list_enum();
    constants();
   ```
4. [variable bindings](variable_bindings/examples.rs)
   ```rust
    mutability();
    scope();
    variable_shadowing();
    declare_first();
    freezing();
   ```
5. [types](types/examples.rs)
   ```rust
    casting();
    literal_sizes();
    type_inference();
    type_aliasing();
   ```
6. [conversions](conversions/examples.rs)
    ```rust
    from_and_into();
    tryfrom_and_tryinto();
    string_conversions();
    ```
7. [expressions](expressions/examples.rs)
8. [flow control](flow_control/examples.rs)
    ```rust
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
    ```
9.  [functions](functions/examples.rs)
    ```rust
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
    ```
10. [modules](modules/examples.rs)
    - `pub(crate)` makes functions visible only within the current crate
    - `pub(in path)`  only visible within the given path
    - `pub(super)` only visible within the parent module
    - `pub(self)` only visible within the current module - same as leaving them private
    - The `self::` keyword refers to the current module scope
    - The `super::` keyword refers to the parent module scope
11. crates (without Cargo)
    ```bash
    rustc --crate-type=lib rary.rs
    rustc executable.rs --extern rary=library.rlib
    ```
12. cargo (basics)
    ```bash
    cargo new foo # A binary
    cargo new --lib bar # A library
    cd foo
    cargo build 
    cargo test
    ```
    ```toml
    [dependencies]
    clap = "4.1.9" # from crates.io
    rand = { git = "https://github.com/rust-lang-nursery/rand" } # from online repo
    bar = { path = "../bar" } # from a path in the local filesystem
    ```
13. [attributes](attributes/examples.rs)
14. [generics](generics/examples.rs)
    ```rust
    basics();
    generic_functions();
    generic_impl();
    generic_traits();
    generic_bounds();
    marker_generic_bounds();
    multiple_generic_bounds();
    where_clauses();
    newtype_idiom();
    associated_types();
    phantom_generic_types();
    ```
15. [scoping rules](scoping/examples.rs)
    ```rust
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
    ```
16. [traits](traits/examples.rs)
```rust
    basics::impl_trait_for_struct();
    auto_derive::comparison_traits();
    box_dyn::return_pointer_to_trait_on_heap();
    operator_overloading::add_foo_plus_bar();
    drop_trait::drop_it();
    iterators::impl_iterator_for_fibonacci();
    impl_traits::return_impl_trait();
    clone_trait::clone();
    trait_disambiguation::qualify();
```
17. [macros_rules!](macros/examples.rs)
```rust
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
```
18. [error handling](error_handling/examples.rs)
    ```rust
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
    ```
    and
    ```rust
    // rustc  examples.rs --cfg 'feature="foo"'
    #[cfg(feature = "foo")]
    fn main() -> Result<(), ParseIntError> {
    ```
19. [std lib types](std_lib_types/examples.rs)
```rust
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
```
20.   [std misc](std_misc/examples.rs)
    
```rust

    threads::spawn_and_join();
    threads::map_reduce();
    threads::channels();
    path_from_string();
    io::create();
    io::open();
    io::read_lines();
    processes::call_process();
    processes::pipes();
    processes::wait();
    filesystem::ops();
    args::parse_args();
    ffi::call_c();
```
21.   [testing](testing/src/lib.rs)
    to run: `cargo test`
22.   [unsafe ops](unsafe_ops/examples.rs)

    ```rust
    
    let raw_p: *const u32 = &10;
    unsafe {
        assert!(*raw_p == 10);
    }
    slice_from_raw_parts();

    // TBH: I need to revise assembly ...
    inline_assembly::nop();
    inline_assembly::mov();
    inline_assembly::mov_add();
    inline_assembly::inout_add();
    inline_assembly::inlateout_add();
    // inline_assembly::explicit_register();    // segmentation fault
    inline_assembly::explicit_registers_mul();   
    inline_assembly::clobbered_registers();      
    inline_assembly::mul_shift_add();            
    inline_assembly::call_foo_clobber_abi(1);
    inline_assembly::register_template_modifier();
    inline_assembly::load_fpu_control_word(1);
    inline_assembly::labels();
    inline_assembly::options();
    ```
23.   compat
24.   meta
    - see https://doc.rust-lang.org/rust-by-example/meta/doc.html 


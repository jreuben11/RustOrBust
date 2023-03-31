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
10. modules
11. crates
12. cargo
13. attributes
14. generics
15. scoping rules
16. traits
17. macros_rules!
18. error handling
19. std lib types
20. std misc
21. testing
22. unsafe ops
23. compat
24. meta

# To run:
```
rustc examples.rs
./examples
```
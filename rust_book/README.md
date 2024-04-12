# Worked examples from the [Rust Book](https://doc.rust-lang.org/book/)

1. [hello_cargo](hello_cargo/README.md) - CH1
2. [guessing_game project](guessing_game/src/main.rs) - CH2 (STDIO, random)
3. [basics](basics/src/main.rs) - CH3-CH11
   -  **modules**
   -  **foundations**: variables_and_mutability, data_types, expressions_and_functions, control_flow
   -  **ownership**: ownership_examples, borrowing, slices
   -  **structs** 
   -  **pattern_matching**
   -  **common_collections**: vectors, strings, hashmaps
   -  **error_handling**: dont_panic ! ;)
   -  **generics**: data_types, traits, lifetimes
   -  **tests**
4. [minigrep project](minigrep/src/main.rs) - CH12
5. [intermediate](intermediate/src/main.rs) - CH13-CH19
   - **FP**: closures, iterators
   - **smart_pointers**: on_heap, ref_counting, interior_mutability, multiple_owner_mut, circular_ref_prevention
   - **concurrency**: thread_spawn, message_passing, shared_state
   - **OOP**: encapsulation, duck_typing, states_as_types
   - **pattern_matching**
   - **advanced**: unsafe, trait_patterns
6. [workspaces](add_workspace/Cargo.toml) - CH14
7. [macros](macros/hello_macro/hello_macro_derive/src/lib.rs) - CH19
8. [web_server multithreading project](web_server/src/lib.rs) - CH20

# from 2nd pass of book:
```bash
cargo check
cargo build --release
cargo doc
```
```rust
gen_range(1..=100)                                               // loop over range
let x = loop { break 5; }                                        // assign value from loop break
{:?}, {:#?}, dbg!   // debug                                     // pretty-print for dbg macro
src/garden/vegetables.rs  ->   use crate::garden::vegetables::asparagus; // module absolute paths
let x: Option<&i32> = v.get(2);                                 // safe Vector access
for i in &mut v { *i += 1; }                                    // mutate derefed vector items
Option<T>.unwrap_or_else(fnOnce)                                 // unwrap an Option, using a closure for None case fallback
iter.map(|x| x+1).filter(|x| x > 0).collect()                   // fluent FP with clousres
let echo = |x| x;                                                // simplest clousre
HashMap<K,V>.sort_by_key(|x| x.y)                              // sort structs in a dictionary
mut args: impl Iterator<Item = String>                           // pass a collection iterator as an arg
sent_messages: RefCell<Vec<String>> = RefCell::new(vec![]);    // interior mutability pattern - runtime borrow checking (use case: trees)
sent_messages.borrow_mut().push(String::from(message));
sent_messages.borrow().len();
```


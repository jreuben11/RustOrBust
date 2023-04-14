# cargo-edit
```bash
cargo install cargo-edit
```

# function_name
```rust
#[macro_use] extern crate function_name;
macro_rules! function_path {() => (concat!(module_path!(), "::", function_name!()))}

...

#[named]
pub fn f() {
    println!("\n{}:", function_path!());
```

# TOC
1. [algorithms](algorithms/src/main.rs)
   ```bash
    cargo add rand rand_distr
   ```
    ```rust
    random::gen_numeric();
    random::gen_numeric_range();
    random::gen_numeric_distribution().unwrap();
    random::gen_random_values_of_t();
    random::gen_alphanumeric_distribution(10);
    random::gen_alphanumeric_from_charset();

    vector_sort();
    ```
2. [command line](cmd_cli/src/main.rs)
   cookbook sample broken - refer to https://docs.rs/clap/latest/clap/

   ```bash
   cargo add clap --features derive
   cargo add ansi_term
   cargo run -- --name xxx --count 5  
   ```
    ```rust
    parse_args();
    coloured_and_styled();
    ```

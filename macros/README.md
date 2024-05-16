# declerative macros
- [](declerative/src/main.rs)
```rust
#![feature(trace_macros)]
#![feature(log_syntax)]

#[macro_use]
mod my_vec;

#[macro_use]
mod greeting;
use crate::greeting::base_greeting_fn;

#[macro_use]
mod generate_get_value;

use crate::account_dsl::Account;
#[macro_use]
mod account_dsl;

#[macro_use]
mod recursive_compose;
use crate::recursive_compose::compose_two;

fn main() {
    custom_vec();
    variadic_greeting();
    recursive_newtype::create();
    use_account_dsl();
    compose_vector_of_fn();
}
```
- [](declerative/src/my_vec.rs)
- [](declerative/src/greeting.rs)
- [](declerative/src/generate_get_value.rs)
- [](declerative/src/account_dsl.rs)
- [](declerative/src/recursive_compose.rs)

# procedural macros
## [](procedural-basic/Cargo.toml)
```toml
[dependencies]
procedural-basic-macro = { path = "./procedural-basic-macro" }
```
## [](procedural-basic/src/main.rs)
```rust
#[macro_use]
extern crate procedural_basic_macro; 

#[derive(Hello)]
struct Example; 

#[derive(Hello)]
enum Pet {
    Cat, 
}

fn main() {
    let e = Example {}; 
    e.hello_world(); 
    let c = Pet::Cat;
    c.hello_world();
}
```
## [](procedural-basic/procedural-basic-macro/Cargo.toml)
```toml
[dependencies]
quote = "1.0.36"
syn = "2.0.63"


[lib]
proc-macro = true
```
## [](procedural-basic/procedural-basic-macro/src/lib.rs)
```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Hello)]
pub fn hello(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);  
    let name = ast.ident; 
    let add_hello_world = quote! {
        impl #name {
            fn hello_world(&self) {
                println!("Hello world")
            }
        }
    };
    add_hello_world.into()
}
```

# declerative macros
- [main.rs](declerative/src/main.rs)
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
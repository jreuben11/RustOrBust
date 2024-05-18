# 4 ways to create macros in rust
1. `macro_rules!`
2. `#[proc_macro_derive(Name)]`
3. `#[proc_macro_attribute]`
4. `#[proc_macro]`

# Declerative Macros
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

trace_macros!(true);

```
- [my_vec.rs](declerative/src/my_vec.rs)
- [greeting.rs](declerative/src/greeting.rs)
- [generate_get_value.rs](declerative/src/generate_get_value.rs)
- [account_dsl.rs](declerative/src/account_dsl.rs)
- [recursive_compose.rs](declerative/src/recursive_compose.rs)
- `macro_rules!` matcher => transcriber clauses
- `cargo expand`

# Procedural Derive Macros
## [Cargo.toml](procedural-basic/Cargo.toml)
```toml
[dependencies]
procedural-basic-macro = { path = "./procedural-basic-macro" }
```
## [main.rs](procedural-basic/src/main.rs)
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
## [macro Cargo.toml](procedural-basic/procedural-basic-macro/Cargo.toml)
```toml
[dependencies]
quote = "1.0.36"
syn = "2.0.63"


[lib]
proc-macro = true
```
## [macro lib.rs](procedural-basic/procedural-basic-macro/src/lib.rs)
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
                println!("Hello", stringify!(#name))
            }
        }
    };
    add_hello_world.into()
}
```
# Procedural Attribute Macros
- [lib.rs](make-public/make-public-macro/src/lib.rs)
```rust
extern crate core;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Colon;
use syn::Data::Struct;
use syn::Fields::Named;
use syn::{parse_macro_input, DataStruct, DeriveInput, Field, FieldsNamed, Ident, Type, Visibility};

impl StructField {
    fn new(field: &Field) -> Self { ... }
}
impl Parse for StructField { 
    fn parse(input: ParseStream) -> Result<Self, syn::Error> { ... }
}
impl ToTokens for StructField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) { .... }
}

#[proc_macro_attribute]
pub fn xxx(attr: TokenStream, item: TokenStream) -> TokenStream { ... }

let ast = parse_macro_input!(item as DeriveInput);
eprintln!("{:#?}", &ast);
```

# procedural function macros
- [main.rs](make-private/src/main.rs)
- [Cargo.toml](make-private/Cargo.toml)
```toml
[dependencies]
make-private-macro = { path = "./make-private-macro" }
function-like-compose-macro = { path = "./function-like-compose-macro"}
```
- [make-private-macro lib.rs](make-private/make-private-macro/src/lib.rs)
```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::Data::Struct;
use syn::Fields::Named;
use syn::__private::{Span, TokenStream2};
use syn::{parse_macro_input, DeriveInput, Type};
use syn::{DataStruct, FieldsNamed, Ident};

fn get_field_info(ast: &DeriveInput) -> Vec<(&Ident, &Type)> { ... }
fn generated_methods(fields: &Vec<(&Ident, &Type)>) -> Vec<TokenStream2> { ... }
fn generate_private_fields(fields: &Vec<(&Ident, &Type)>) -> Vec<TokenStream2> { ... }

#[proc_macro]
pub fn private(item: TokenStream) -> TokenStream { ... }
```
- [function-like-compose-macro lib.rs](make-private/function-like-compose-macro/src/lib.rs)
```rust
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Ident, parse_macro_input, Token};

struct ComposeInput {
    expressions: Punctuated<Ident, Token!(.)>,
}
impl Parse for ComposeInput {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> { ... }
}
impl ToTokens for ComposeInput {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) { ... }
}

#[proc_macro]
pub fn compose(item: TokenStream) -> TokenStream {
    let ci: ComposeInput = parse_macro_input!(item);
    quote!(
      {
        ...
        #ci
      }
    ).into()
}
```
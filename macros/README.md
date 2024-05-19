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

# Procedural Function Macros
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

# Builder pattern + Testing
- [workspace](builder/Cargo.toml)
```toml
[workspace]
resolver = "2"
members = [
"builder-macro", 
"builder-code", 
"builder-usage" 
]
```
## builder-usage
- [Cargo.toml](builder/builder-usage/Cargo.toml)
```toml
[dependencies]
builder-macro = { path = "../builder-macro" }
[dev-dependencies]
trybuild = "1.0.96"
```
- [main.rs](builder/builder-usage/src/main.rs)
```rust
use builder_macro::Builder;

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_generate_builder_for_struct_with_no_properties() { ... }
    #[test]
    fn should_generate_builder_for_struct_with_one_property() { ... }
    #[test]
    fn should_generate_builder_for_struct_with_two_properties() { ... }
    #[test]
    fn should_generate_builder_for_struct_with_multiple_properties() { ... }
    #[test]
    #[should_panic]
    fn should_panic_when_field_is_missing() { ... }
}
```
- [compilation_tests.rs](builder/builder-usage/tests/compilation_tests.rs)
```rust
#[test]
fn should_not_compile() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/fails/*.rs");
}
```
## builder-macro
- [Cargo.toml](builder/builder-macro/Cargo.toml)
```toml
[dependencies]
builder-code = { path = "../builder-code" }

[lib]
proc-macro = true
```
- [lib.rs](builder/builder-macro/src/lib.rs)
```rust
use builder_code::create_builder;
use proc_macro::TokenStream;

#[proc_macro_derive(Builder)]
pub fn builder(item: TokenStream) -> TokenStream {
    create_builder(item.into()).into()
}

```

## builder-code
- [Cargo.toml](builder/builder-code/Cargo.toml)
```toml
[dependencies]
proc-macro2 = "1.0.82"
quote = "1.0.36"
syn = { version = "2.0.64", features = ["extra-traits"] }
```
- [fields.rs](builder/builder-code/src/fields.rs)
```rust
use quote::quote;
use syn::__private::TokenStream2;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Field, Ident, Type};

fn get_name_and_type<'a>(f: &'a Field) -> (&'a Option<Ident>, &'a Type) { ... }
pub fn builder_field_definitions(fields: &Punctuated<Field, Comma>,) -> impl Iterator<Item = TokenStream2> + '_ { ... }
pub fn original_struct_setters(fields: &Punctuated<Field, Comma>,) -> impl Iterator<Item = TokenStream2> + '_ { ... }
pub fn builder_methods(fields: &Punctuated<Field, Comma>,) -> impl Iterator<Item = TokenStream2> + '_ { ... }
pub fn builder_init_values(fields: &Punctuated<Field, Comma>,) -> impl Iterator<Item = TokenStream2> + '_ { ... }

#[cfg(test)]
mod tests {
    use proc_macro2::Span;
    use syn::{FieldMutability, Path, PathSegment, TypePath, Visibility};
    use super::*;
    #[test]
    fn get_name_and_type_give_back_name() { ... }
}
```
- [lib.rs](builder/builder-code/src/lib.rs)
```rust
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Data::Struct;
use syn::DeriveInput;
use syn::Fields::Named;
use syn::{DataStruct, FieldsNamed, Ident};

mod fields;
use fields::{
    builder_field_definitions, builder_init_values, builder_methods, original_struct_setters,
};

pub fn create_builder(item: TokenStream) -> TokenStream { ... }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn builder_struct_name_should_be_present_in_output() { ... }
    #[test]
    fn builder_struct_with_expected_methods_should_be_present_in_output() { ... }
}
```

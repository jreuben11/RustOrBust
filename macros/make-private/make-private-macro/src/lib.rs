use proc_macro::TokenStream; 
use syn::{parse_macro_input, DeriveInput}; 
use quote::quote;

#[proc_macro] 
pub fn private(item: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(item as DeriveInput); 
  let name = ast.ident; 
  quote!(
    struct #name {} 
    impl #name {} 
  ).into()
}
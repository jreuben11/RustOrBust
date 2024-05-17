extern crate core;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Colon;
use syn::Data::Struct;
use syn::Fields::Named;
use syn::{parse_macro_input, DataStruct, DeriveInput, Field, FieldsNamed, Ident, Type, Visibility};

struct StructField {
    name: Ident,
    ty: Type,
}

impl StructField {
    fn new(field: &Field) -> Self {
        Self {
            name: field.ident.as_ref().unwrap().clone(),
            ty: field.ty.clone(),
        }
    }
}

impl ToTokens for StructField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let n = &self.name;
        let t = &self.ty;
        quote!(pub #n: #t).to_tokens(tokens)
    }
}

struct StructField2 {
    name: Ident,
    ty: Ident, // PROBLEMATIC - how to impl ToTokens ???
}

impl ToTokens for StructField2 {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let n = &self.name;
        let t = &self.ty;
        quote!(pub #n: #t).to_tokens(tokens)
    }
}

impl Parse for StructField2 { 
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let _vis: Result<Visibility, _> = input.parse(); 
      let list = Punctuated::<Ident, Colon>::parse_terminated(input).unwrap(); 
      Ok(StructField2 {
        name: list.first().unwrap().clone(), 
        ty: list.last().unwrap().clone(), 
      }) 
    }
  }

#[proc_macro_attribute]
pub fn public(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    eprintln!("{:#?}", &ast);
    let name = ast.ident;
    let fields = match ast.data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => unimplemented!("only works for structs with named fields"),
    };
    // METHOD 1: inline
    let _builder_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! { pub #name: #ty }
    });

    // METHOD 2: toTokens
    let _builder_fields = fields.iter().map(StructField::new);

    // Method 3: toTokens + parse
    let builder_fields = fields.iter().map(|f| {
        syn::parse2::<StructField2>(f.to_token_stream()).unwrap()
    }); 

    let public_version = quote! {
      pub struct #name {
        #(#builder_fields,)*
      }
    };
    public_version.into()
}

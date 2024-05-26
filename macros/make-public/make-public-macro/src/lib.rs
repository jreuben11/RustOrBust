extern crate core;
use std::iter::Map;

use proc_macro::TokenStream;
use quote::{__private, quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::{Iter, Punctuated};
use syn::token::{Colon, Comma};
use syn::Data::{Struct, Enum};
use syn::Fields::{Named, Unnamed};
use syn::{
    parse_macro_input, DataEnum, DataStruct, DeriveInput, Field, FieldsNamed, FieldsUnnamed, Ident, Type, Variant, Visibility
};

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

fn named_fields_public(
    fields: &Punctuated<Field, Comma>,
) -> Map<Iter<Field>, fn(&Field) -> quote::__private::TokenStream> {
    fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! { pub #name: #ty }
    })
}

fn unnamed_fields_public(
    fields: &Punctuated<Field, Comma>,
) -> Map<Iter<Field>, fn(&Field) -> quote::__private::TokenStream> {
    fields.iter().map(|f| {
        let ty = &f.ty;
        quote! { pub #ty }
    })
}

fn generate_named_output<'a>(
    struct_name: Ident,
    builder_fields: Map<Iter<'a, Field>, fn(&'a Field) -> __private::TokenStream>,
) -> quote::__private::TokenStream {
    quote!(
        pub struct #struct_name {
            #(#builder_fields,)*
        }
    )
}

fn generate_unnamed_output<'a>(
    struct_name: Ident,
    builder_fields: Map<Iter<'a, Field>, fn(&'a Field) -> __private::TokenStream>,
) -> quote::__private::TokenStream {
    quote!(
        pub struct #struct_name(
            #(#builder_fields,)*
        );
    )
}

fn generate_enum_output(enum_name: Ident, variants: &Punctuated<Variant, Comma>) -> quote::__private::TokenStream {
    let as_iter = variants.into_iter();

    quote!(
        pub enum #enum_name {
            #(#as_iter,)*
        }
    )
}

#[proc_macro_attribute]
pub fn public_struct(_attr: TokenStream, item: TokenStream) -> TokenStream {
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
    let builder_fields = fields
        .iter()
        .map(|f| syn::parse2::<StructField2>(f.to_token_stream()).unwrap());

    let public_version = quote! {
      pub struct #name {
        #(#builder_fields,)*
      }
    };
    public_version.into()
}


#[proc_macro_attribute]
pub fn public(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    // eprintln!("{:#?}", &ast);
    let name = ast.ident;
    let attributes = &ast.attrs;
    let basic_output = match ast.data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => {
            let f = named_fields_public(named);
            generate_named_output(name, f)
        }
        Struct(DataStruct {
            fields: Unnamed(FieldsUnnamed { ref unnamed, .. }),
            ..
        }) => {
            let f = unnamed_fields_public(unnamed);
            generate_unnamed_output(name, f)
        }
        Enum(DataEnum { ref variants, .. }) => {
            generate_enum_output(name, variants)
        },
        _ => unimplemented!("only works for structs and enums"),
    };
    
    quote!(
        #(#attributes)*
        #basic_output
    ).into()
}

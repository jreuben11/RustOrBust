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

pub fn create_builder(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse2(item).unwrap();
    let name: Ident = ast.ident;
    let builder = format_ident!("{}Builder", name);
    let fields = match ast.data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => unimplemented!("only implemented for structs"),
    };
    let builder_fields = builder_field_definitions(fields);
    let builder_inits = builder_init_values(fields);
    let builder_methods = builder_methods(fields);
    let set_fields = original_struct_setters(fields);
    quote! {
      struct #builder {
        #(#builder_fields,)*
      }
      impl #builder {
        #(#builder_methods)*
        pub fn build(self) -> #name {
          #name {
            #(#set_fields,)*
          }
        }
      }
      impl #name {
        pub fn builder() -> #builder {
          #builder {
            #(#builder_inits,)*
          }
        }
      }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn builder_struct_name_should_be_present_in_output() {
        let input = quote! {
          struct StructWithNoFields {}
        };
        let actual = create_builder(input);
        assert!(actual.to_string().contains("StructWithNoFieldsBuilder"));
    }

    #[test]
    fn builder_struct_with_expected_methods_should_be_present_in_output() {
        let input = quote! {
          struct StructWithNoFields {}
        };
        let expected = quote! {
            struct StructWithNoFieldsBuilder { }
            impl StructWithNoFieldsBuilder {
                pub fn build (self) -> StructWithNoFields {
                    StructWithNoFields { }
                }
            }
            impl StructWithNoFields {
                pub fn builder () -> StructWithNoFieldsBuilder {
                    StructWithNoFieldsBuilder { }
                }
            }
        };
        let actual = create_builder(input);
        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[ignore]
    #[test]
    fn assert_with_parsing() {
        let input = quote! {
          struct StructWithNoFields {}
        };
        let actual = create_builder(input);
        let derived: DeriveInput = syn::parse2(actual).unwrap(); // will no longer work
        let name = derived.ident;
        assert_eq!(name.to_string(), "StructWithNoFieldsBuilder");
    }
}

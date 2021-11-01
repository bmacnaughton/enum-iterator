
extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;

use syn::{Ident, Body, Variant, VariantData};
use proc_macro::TokenStream;

#[proc_macro_derive(EnumIterator)]
pub fn enum_iterator(input: TokenStream) -> TokenStream {
    // construct stream representation of the type definition
    let s = input.to_string();

    // parse the string rep
    let ast = syn::parse_macro_input(&s).unwrap();

    let name = &ast.ident;
    let gen = match ast.body {
      Body::Enum(ref variants) => impl_enum_iterator(name, variants),
      Body::Struct(_) => panic!("#[derive(EnumIterator) is not defined for structs")
    };
    gen.parse().unwrap()
}

fn impl_enum_iterator(name: &Ident, variants: &[Variant]) -> quote::Tokens {
    let interface = quote::Ident::from(format!("_EnumIterator{}", name));
    let match_usize = match_usize(&name, variants);
    let size = variants.len();

    quote! {
        #[derive(Debug, Default)]
        pub struct #interface {
            count: usize
        }

        impl #name {
            fn enum_iterator() -> #interface {
                #interface:: default()
            }
        }

        impl #interface {
            fn from_usize(n: usize) -> #name {
                match n {
                    #(#match_usize)*
                    _ => unreachable!(), // he thinks
                }
            }
        }

        impl ::std::iter::Iterator for #interface {
            type Item = #name;
            fn next(&mut self) -> Option<Self::Item> {
                if self.count >= #size {
                    return None;
                }
                let result = #interface::from_usize(self.count);
                self.count += 1;
                Some(result)
            }
        }
    }
}

fn match_usize(name: &Ident, variants: &[Variant]) -> Vec<quote::Tokens> {
    let mut result = Vec::new();

    for (ix, variant) in variants.iter().enumerate() {
        let id = &variant.ident;
        let new = match variant.data {
            VariantData::Unit => quote! {
                #ix => #name::#id,
            },
            VariantData::Tuple(ref fields) => {
                let types = fields.iter().map(|f| &f.ty);
                quote! {
                    #ix => #name::#id( #(#types::default(),)*),
                }
            },
            VariantData::Struct(ref fields) => {
                let items = fields.iter().map(|f| {
                    let ident = &f.ident;
                    let ty = &f.ty;
                    quote! {
                        #ident: #ty::default()
                    }
                });
                quote! {
                    #ix => #name::#id {#(#items,)*},
                }
            }
        };
        result.push(new);
    }

    result
}

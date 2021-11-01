//#![feature(proc_macro, proc_macro_lib)]
extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro(input: TokenStream) -> TokenStream {
    // construct stream representation of the type definition
    let s = input.to_string();

    // parse the string rep
    let ast = syn::parse_macro_input(&s).unwrap();

    // build the impl
    let gen = impl_hello_macro(&ast);

    // return the generated impl
    gen.parse().unwrap()
}

fn impl_hello_macro(ast: &syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl #name {
            fn hello_macro() {
                println!("Hello Macro! My name is {}", stringify!(#name));
            }
        }
    }
}

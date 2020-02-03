extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::{quote, ToTokens};

#[proc_macro_derive(ResxPath)]
pub fn resx_path(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let tokens = impl_resx_path(&ast);
    tokens.to_token_stream().into()
}

fn impl_resx_path(ast: &syn::DeriveInput) -> impl ToTokens {
    let name = format!("{}", ast.ident);
    quote! {
        impl ResxPath for #name {
            fn path(self) -> String {
                self.0
            }
            fn new(path: String) -> Self {
                #name(path)
            }
        }
    }
}

#[proc_macro_derive(ResxRB)]
pub fn resx_rb(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let tokens = impl_resx_rb(&ast);
    tokens.to_token_stream().into()
}

fn impl_resx_rb(ast: &syn::DeriveInput) -> impl ToTokens {
    let name = format!("{}", ast.ident);
    quote! {
        impl ResxRB for #name {}
    }
}


#[proc_macro_derive(ResxInstanceRB)]
pub fn resx_instance_rb(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let tokens = impl_resx_instance_rb(&ast);
    tokens.to_token_stream().into()
}

fn impl_resx_instance_rb(ast: &syn::DeriveInput) -> impl ToTokens {
    let name = format!("{}", ast.ident);
    quote! {
        impl ResxInstanceRB for #name {}
    }
}

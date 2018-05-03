extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(ResxPath)]
pub fn resx_path(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_resx_path(&ast);
    gen.parse().unwrap()
}

fn impl_resx_path(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
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
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_resx_rb(&ast);
    gen.parse().unwrap()
}

fn impl_resx_rb(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl ResxRB for #name {}
    }
}


#[proc_macro_derive(ResxInstanceRB)]
pub fn resx_instance_rb(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_resx_instance_rb(&ast);
    gen.parse().unwrap()
}

fn impl_resx_instance_rb(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl ResxInstanceRB for #name {}
    }
}

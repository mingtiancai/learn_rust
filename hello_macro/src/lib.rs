extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quete::quete;
use syn;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub trait HelloMacro {
    fn hello_macro();
}

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwarp();
    impl_hello_macro(&ast);
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quete{
        impl HelloMacro for #name{
            fn hello_macro(){
                println!("Hello,Macro! My name is {}",stringify!(#name));
            }
        }
    };
    
    gen.into();
}

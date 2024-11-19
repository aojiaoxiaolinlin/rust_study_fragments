extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;
// 自定义derive宏，derive宏是自能拥有struct和enum
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 构建rust代码的语法树
    let ast: DeriveInput = syn::parse(input).unwrap();
    // 构建要返回的rust代码
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };
    // 将rust代码的语法树转换为字符串
    gen.into()
}

#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    // 构建rust代码的语法树
    let input = syn::parse_macro_input!(item as syn::Ident);
    // 构建要返回的rust代码
    let gen = quote! {
        #input
    };
    // 将rust代码的语法树转换为字符串
    gen.into()
}

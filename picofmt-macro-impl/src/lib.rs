extern crate proc_macro;

//use crate::proc_macro::{TokenStream, TokenTree};
use proc_macro2::TokenStream as TokenStream;
use proc_macro2::TokenTree as TokenTree;
use quote::quote;
use syn::{parse_macro_input, Expr, DeriveInput};
use proc_macro_hack::proc_macro_hack;
use std::any::Any;

use std::fs::File;
use std::io::prelude::*;

#[proc_macro_hack]
pub fn hello_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    //println!("{:?}", input);
    let input = TokenStream::from(input);
    println!("{:?}", input);

    let mut file = File::create("/Users/roman/proc-out.txt");
    if file.is_ok() {
        write!(file.unwrap(), "{:#?}", input);
    }

    //let expr = parse_macro_input!(input as Expr);
//    let ast: DeriveInput = syn::parse(input).unwrap();
//    println!("{:?}", ast);
//    for tok in input.into_iter() {
//        match tok {
//            TokenTree::Group(tt) => {println!("group: {}", tt);},
//            TokenTree::Punct(tt) => {println!("punct: {}", tt);},
//            TokenTree::Ident(tt) => {println!("ident: {}", tt);},
//            TokenTree::Literal(tt) => {println!("lit: {}", tt);},
//            _ => {println!("def_")}
//        };
//    }

    let toks: Vec<TokenTree> = input.into_iter().collect();
    println!("{:?}", toks[2]);
    let id = &toks[2];

    let var = &toks[3];
//
//    let ident = if let TokenTree::Ident(tt) = &toks[2] {
//        tt
//    } else {
//        panic!("err");
//    };

    proc_macro::TokenStream::from(quote! {
        //let macro_x = 123;
        {
        #id = #id + 1;
        let y = 123;
        //println!("test {}", #id);
        #[used]
        #[link_section = ".picofmt"]
        static PROC_MACRO_MAGIC: [u8; 6] = [b'm',b'a',b'g',b'i',b'c', b'\x01'];
        }
        //let #var = 10;
    })
}

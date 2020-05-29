#![no_std]

use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack]
pub use biptt_macro_impl::hello_macro;
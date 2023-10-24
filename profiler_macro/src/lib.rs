#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;
extern crate proc_macro2;

use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn run_time(
    _attr: proc_macro::TokenStream,
    func: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let func = parse_macro_input!(func as ItemFn);
    let func_vis = &func.vis; // like pub
    let func_block = &func.block; // { some statement or expression here }

    let func_decl = func.sig;
    let func_name = &func_decl.ident; // 函数名
    let func_generics = &func_decl.generics;
    let func_inputs = &func_decl.inputs;
    let func_output = &func_decl.output;

    let caller = quote! {
        // rebuild the function, add a func named is_expired to check user login session expire or not.
        #func_vis fn #func_name #func_generics(#func_inputs) #func_output {
            // TODO porting start_timer things  here.
            use ark_std::{end_timer, start_timer};
            let start=  start_timer!(|| "start_time");
            #func_block
            end_timer!(start);
        }
    };

    caller.into()
}

// my-macro/src/lib.rs

#[proc_macro_attribute]
pub fn show_streams(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());
    item
}

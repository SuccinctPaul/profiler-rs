//! To use this macro,
//!
//! * First thing to do is config `Cargo.toml` :
//!
//! ```toml
//! [dependencies]
//! profiler_macro = {path = "../profiler_macro"}
//! ark-std = { version = "0.4.0", optional = true }
//!
//! [features]
//! profile = ["ark-std/print-trace"] # Must use this feature!
//! ```
//!
//!
//! * Second to declare on target function:
//! ```rust
//!  extern crate profiler_macro;
//!  use profiler_macro::time_profiler;
//!
//!  #[test]
//!  fn test() {
//!      #[time_profiler()] // default: with the function's name
//!      fn outer(t: u64) {
//!          inner(t-1);
//!          println!("outer:{t}");
//!      }
//!
//!      #[time_profiler("inner")]
//!      fn inner(t: u64) {
//!          println!("inner:{t}");
//!      }
//!
//!      outer(4);
//!  }
//! ```

#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;
extern crate proc_macro2;

use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// Timer profiler macro. Cooperate with `ark-std`
#[proc_macro_attribute]
pub fn time_profiler(
    attr: proc_macro::TokenStream,
    func: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr_name = attr.to_string();

    let func = parse_macro_input!(func as ItemFn);
    let func_vis = &func.vis;
    let func_block = &func.block; // { some statement or expression here }

    let func_decl = func.sig;
    let func_name = &func_decl.ident;
    let func_generics = &func_decl.generics;
    let func_inputs = &func_decl.inputs;
    let func_output = &func_decl.output;

    let name = if attr_name.is_empty() {
        func_name.to_string()
    } else {
        attr_name.replace("\"", "")
    };

    let caller = quote! {
        // rebuild the function, add a func named is_expired to check user login session expire or not.
        #func_vis fn #func_name #func_generics(#func_inputs) #func_output {
            #[cfg(feature = "profile")]
            use ark_std::{end_timer, start_timer};

            #[cfg(feature = "profile")]
            let start=  ark_std::start_timer!(|| #name);

            #func_block

            #[cfg(feature = "profile")]
            ark_std::end_timer!(start);
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

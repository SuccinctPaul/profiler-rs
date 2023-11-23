extern crate proc_macro2;
extern crate quote;
extern crate syn;

use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// Timer profiler macro. Cooperate with `ark-std`
#[proc_macro_attribute]
pub fn time_profiler(
    attr: proc_macro::TokenStream,
    stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr_name = attr.to_string();

    let func = parse_macro_input!(stream as ItemFn);
    let func_vis = &func.vis;
    let func_block = &func.block; // { some statement or expression here }
    let func_attr = &func.attrs;
    let func_sig = func.sig;

    let func_name = &func_sig.ident;

    let log_name = if attr_name.is_empty() {
        func_name.to_string()
    } else {
        attr_name.replace('\"', "")
    };

    let caller = quote! {
        // rebuild the function, add a func named is_expired to check user login session expire or not.
        // #func_vis fn #func_name #func_generics(#func_inputs) #func_output {
        #(#func_attr)* #func_vis #func_sig {
            #[cfg(feature = "profiler")]
            use ark_std::{end_timer, start_timer};

            #[cfg(feature = "profiler")]
            let start=  ark_std::start_timer!(|| #log_name);

            let result =  #func_block;

            #[cfg(feature = "profiler")]
            ark_std::end_timer!(start);

            result
        }
    };

    caller.into()
}

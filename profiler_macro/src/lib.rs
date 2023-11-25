#![doc = include_str!("../../README.md")]

extern crate proc_macro2;
extern crate quote;
extern crate syn;

use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// Timer flamegraph macro. Cooperate with `ark-std`
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

/// Flamegraph profiler macro. Cooperate with `tracing-flame` and `inferno`
#[proc_macro_attribute]
pub fn flamegraph_profiler(
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

            #[cfg(feature = "flamegraph")]
            {
                use tracing::{span, Level};
                span!(Level::TRACE, &#log_name).in_scope(|| {
                    #func_block
                 })
            }

            #[cfg(not(feature = "flamegraph"))]
            #func_block
        }
    };

    caller.into()
}

/// Flamegraph profiler macro. Cooperate with `tracing-flame` and `inferno`
#[proc_macro_attribute]
pub fn trace_flamegraph_main(
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


            #[cfg(feature = "flamegraph")]
            {
                use std::{
                    fs::File,
                    io::{BufReader, BufWriter},
                    path::{Path, PathBuf},
                };
                use tracing::{span, subscriber::set_global_default, Level};
                use tracing_flame::FlameLayer;
                use tracing_subscriber::{prelude::*, registry::Registry};

                const PERF_DATA: &str = "/tmp/tracing-flame-inferno.flame.folded";
                const FLAMEGRAPH_FILE: &str = "tracing-flame-inferno.svg";

                fn setup_global_collector() -> impl Drop {

                    let (flame_layer, _guard) = FlameLayer::with_file(PERF_DATA).unwrap();

                    let collector = Registry::default().with(flame_layer);

                    set_global_default(collector).unwrap();

                    _guard
                }

                fn make_flamegraph() {
                    let inf = File::open(PERF_DATA).unwrap();
                    let reader = BufReader::new(inf);
                    // flamegraph.svg
                    println!("outputting flamegraph to {}", FLAMEGRAPH_FILE);
                    let out = File::create(FLAMEGRAPH_FILE).unwrap();
                    let writer = BufWriter::new(out);

                    let mut opts = inferno::flamegraph::Options::default();
                    inferno::flamegraph::from_reader(&mut opts, reader, writer).unwrap();
                }

                // setup the flame layer
                let guard = setup_global_collector();
                span!(Level::TRACE, &#log_name).in_scope(|| {
                    #func_block
                 });


                // drop the guard to make sure the layer flushes its output then read the
                // output to create the flamegraph
                drop(guard);

                make_flamegraph();
            }

            #[cfg(not(feature = "flamegraph"))]
            #func_block
        }
    };

    caller.into()
}

#![allow(unused_imports)]
#![allow(unused)]

mod docs_test;
mod flamegraph_test;
mod trait_test;

use profiler_macro::{time_profiler, trace_flamegraph_main};

// dev/product:
// cargo test test -- --nocapture
//
// profile:
//      cargo test test --features profiler -- --nocapture
#[test]
fn test() {
    #[time_profiler()] // default: with the function's name
    fn outer(t: u64) {
        let res = inner(t - 1);
        println!("outer:{t}, from inner: {res}");
    }

    #[time_profiler("inner")]
    fn inner(t: u64) -> u64 {
        println!("inner:{t}");
        1
    }

    outer(4);
}

// cargo test flamegraph_test --features flamegraph -- --nocapture
#[test]
#[trace_flamegraph_main()]
fn flamegraph_test() {
    // have_a_pet(Dog);
    println!("hello, world");
}

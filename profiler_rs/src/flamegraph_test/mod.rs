use profiler_macro::trace_flamegraph_main;

// cargo test flamegraph_test --features flamegraph -- --nocapture
#[trace_flamegraph_main()]
#[test]
fn flamegraph_test() {
    // have_a_pet(Dog);
    println!("hello, world");
}

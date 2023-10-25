#[allow(unused_imports)]
use profiler_macro::time_profiler;

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

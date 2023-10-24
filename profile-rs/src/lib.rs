extern crate profiler_macro;

use profiler_macro::time_profiler;

#[test]
fn test() {
    #[time_profiler()] // default: with the function's name
    fn outer(t: u64) {
        inner(t - 1);
        println!("outer:{t}");
    }

    #[time_profiler("inner")]
    fn inner(t: u64) {
        println!("inner:{t}");
    }

    outer(4);
}

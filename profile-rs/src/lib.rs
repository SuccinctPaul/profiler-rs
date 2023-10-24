extern crate profiler_macro;

use profiler_macro::time_profiler;

#[test]
fn test() {
    // default: with the function's name
    #[time_profiler()]
    fn outer(t: u64) {
        for i in 0..t {
            println!("outer:{i}");
            inner(i);
        }
    }

    #[time_profiler("inner")]
    fn inner(t: u64) {
        println!("inner:{t}");
    }

    outer(4);
    outer(2);
}

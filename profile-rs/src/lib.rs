extern crate profiler_macro;
use profiler_macro::run_time;

#[cfg(test)]
mod test {
    use profiler_macro::{run_time, show_streams};

    #[run_time]
    fn outer(t: u64) {
        println!("start");
        for i in 0..t {
            for j in 0..i {
                println!("outer:{i}_{j}");
            }
        }
        println!("finish");
    }

    #[run_time]
    fn inner(t: u64) {
        println!("inner:{t}");
    }

    #[test]
    fn test_timer() {
        outer(4);
        outer(2);
    }

    #[test]
    fn test_try() {
        // Example: Basic function
        #[show_streams]
        fn invoke1() {}
        // out: attr: ""
        // out: item: "fn invoke1() {}"

        // Example: Attribute with input
        #[show_streams(bar)]
        fn invoke2() {}
        // out: attr: "bar"
        // out: item: "fn invoke2() {}"

        // Example: Multiple tokens in the input
        #[show_streams(multiple => tokens)]
        fn invoke3() {}
        // out: attr: "multiple => tokens"
        // out: item: "fn invoke3() {}"

        // Example:
        #[show_streams { delimiters }]
        fn invoke4() {}
        // out: attr: "delimiters"
        // out: item: "fn invoke4() {}"
        invoke1();
        invoke4();
    }
}

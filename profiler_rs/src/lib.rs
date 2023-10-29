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

#[test]
fn test_trait() {
    trait Animal {
        // For now, time_profiler can't be used for trait.
        fn name() -> String;
    }

    struct Dog;
    impl Dog {
        #[time_profiler()] // default: with the function's name
        pub fn shout() {
            println!("wowowoow");
            println!("I'm a {:?}", Self::name());
        }
    }
    impl Animal for Dog {
        #[time_profiler()] // default: with the function's name
        fn name() -> String {
            String::from("dog")
        }
    }

    struct Cat;
    impl Cat {
        #[time_profiler("talk")]
        pub fn talk() {
            println!("mimimi");
            println!("I'm a {:?}", Self::name());
        }
    }
    impl Animal for Cat {
        #[time_profiler("cat_name")] // default: with the function's name
        fn name() -> String {
            String::from("cat")
        }
    }

    Dog::shout();
    println!();
    Cat::talk();
}

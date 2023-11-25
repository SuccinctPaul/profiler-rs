#![deny(missing_docs)]

use profiler_macro::time_profiler;
use std::fmt::Debug;

/// I have doc comments!
#[time_profiler()]
fn say_something_for_doc() {
    println!("Say something: I have doc comments!");
}

/// I have doc comments!
struct Tom<T: Debug> {
    age: T,
}

impl<T: Debug> Tom<T> {
    fn new(age: T) -> Self {
        Self { age }
    }

    /// I have doc comments!
    #[time_profiler()]
    fn self_intro(&self) {
        println!("I'm Tom, at {:?}", self.age);
    }
}

// cargo test docs_test --features profiler  -- --nocapture
#[test]
fn docs_test() {
    say_something_for_doc();
    Tom::new(10).self_intro();
}

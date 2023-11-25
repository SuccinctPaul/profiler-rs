use std::{thread::sleep, time::Duration};

use profiler_macro::{flamegraph_profiler, trace_flamegraph_main};

trait Animal {
    // For now, time_profiler can't be used for trait.
    fn name() -> String;

    #[flamegraph_profiler("Default Name")]
    fn default() -> String {
        sleep(Duration::from_millis(50));
        "Groot".to_string()
    }
}

struct Dog;
impl Dog {
    #[flamegraph_profiler()]
    pub fn shout() {
        sleep(Duration::from_millis(50));

        println!("wowowoow");
        println!("I'm a {:?}", Self::name());
    }
}
impl Animal for Dog {
    #[flamegraph_profiler()] // default: with the function's name
    fn name() -> String {
        String::from("dog")
    }
}

struct Cat;
impl Cat {
    #[flamegraph_profiler("talk")]
    pub fn talk() {
        println!("mimimi");
        println!("I'm a {:?}", Self::name());
    }
}
impl Animal for Cat {
    #[flamegraph_profiler("cat_name")] // default: with the function's name
    fn name() -> String {
        String::from("cat")
    }
}

struct Groot;
impl Groot {
    #[flamegraph_profiler()]
    fn self_instro() {
        println!("I'm the {:?}", Self::name());
    }
}

impl Animal for Groot {
    #[flamegraph_profiler("Groot_name")]
    fn name() -> String {
        Self::default()
    }
}

#[flamegraph_profiler()]
fn have_a_pet<A>(animal: A)
where
    A: Animal,
{
    println!("pet: {:?}", A::name());
}

// cargo run --example flamegraph_simple --features flamegraph --release -- --nocapture
#[trace_flamegraph_main()]
fn main() {
    Dog::shout();
    Cat::talk();
    Groot::self_instro();

    have_a_pet(Dog);
}

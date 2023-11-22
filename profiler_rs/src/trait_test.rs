use profiler_macro::time_profiler;

trait Animal {
    // For now, time_profiler can't be used for trait.
    fn name() -> String;

    #[time_profiler("Default Name")]
    fn default() -> String {
        "Groot".to_string()
    }
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

struct Groot;
impl Groot {
    #[time_profiler()]
    fn self_instro() {
        println!("I'm the {:?}", Self::name());
    }
}

impl Animal for Groot {
    #[time_profiler("Groot_name")]
    fn name() -> String {
        Self::default()
    }
}

// cargo test trait_test --features profiler -- --nocapture
#[test]
fn trait_test() {
    Dog::shout();
    Cat::talk();
    Groot::self_instro();
}

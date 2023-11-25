# USE GUIDE


## How to use `time_profiler` to profile time?
> Demo can be found [here](https://github.com/ChengYueJia/profiler-rs/tree/main/profiler_rs/src/timer_profiler_test/).
1. Config `Cargo.toml` :
```toml
[dependencies]
profiler_macro = {git = "https://github.com/ChengYueJia/profiler-rs.git"}
ark-std = { version = "0.4.0", optional = true }

[features]
profiler = ["ark-std/print-trace"] # Must use this feature!
```

2. Second to declare macro `#[time_profiler("log_name")] `on function:
   The `log_name` can be null, macro will use the function's name as `log_name`.
```rust
use profiler_macro::time_profiler;

#[time_profiler()] // default: with the function's name
fn outer(t: u64) {
    inner(t-1);
}

#[time_profiler("inner")]
fn inner(t: u64) {
    // impl
}

```

3. Profile it:
```bash
 cargo test test --features profiler -- --nocapture
```
* Normal run:
```bash
 cargo test test -- --nocapture
```

## How to use `flamegraph_profiler` and `trace_flamegraph_main` to generate marked flamegraph?
> Demo can be found [here](https://github.com/ChengYueJia/profiler-rs/tree/main/profiler_rs/examples/flamegraph_simple.rs).
1. Config `Cargo.toml` :
```toml
[dependencies]
profiler_macro = {git = "https://github.com/ChengYueJia/profiler-rs.git"}
# tracing+flamegraph
inferno = { version = "0.11.6", optional = true }
tracing = { version = "0.1.40", default-features = false, features = ["std"], optional = true }
tracing-flame = { version = "0.2.0", optional = true }
tracing-subscriber = {  version = "0.3.18", default-features = false, features = ["registry", "fmt", "smallvec"] , optional =  true}

[features]
# flamegraph feature
flamegraph = [ "inferno", "tracing", "tracing-subscriber", "tracing-flame"]
```

2. Second to declare macro `#[flamegraph_profiler("log_name")] `on function:
   The `log_name` can be null, macro will use the function's name as `log_name`.
> TIPS: Use the default `log_name`(function's name) will be better.
```rust
use profiler_macro::time_profiler;

#[flamegraph_profiler()] // default: with the function's name
fn outer(t: u64) {
    inner(t-1);
}

#[flamegraph_profiler("inner")]  
fn inner(t: u64) {
    // impl
}

```

3. To declare macro `#[trace_flamegraph_main("log_name")] `on main/test function:
```rust
// 
#[trace_flamegraph_main()]
#[test]
fn flamegraph_test() {
    // have_a_pet(Dog);
    println!("hello, world");
}
```

3. Flaemgraph it:
```bash
cargo test flamegraph_test --features flamegraph -- --nocapture
```
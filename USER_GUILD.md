## How to use?
> Demo can be found [here](https://github.com/ChengYueJia/profiler-rs/tree/main/profiler_macro).
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

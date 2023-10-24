#[cfg(test)]
mod test {
    use profiler_macro::run_time;

    #[run_time]
    fn deco(t: u64) {
        println!("1");
        let secs = Duration::from_secs(t);
        thread::sleep(secs);
        println!("deco");
    }

    #[test]
    fn test() {
        deco(4);
        deco(2);
    }
}

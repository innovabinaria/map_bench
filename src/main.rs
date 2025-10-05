use map_bench::{cli::dispatch_or_run_all, config::Config};

fn main() {
    // CLI-compatible with the previous version:
    // - No args: run all three benchmarks with defaults.
    // - With args: `map_bench <hashmap|btreemap|vec> N Q SEED`.
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        let cfg = Config::default_run_all();
        dispatch_or_run_all(None, cfg);
        return;
    }

    if args.len() < 5 {
        eprintln!("Usage: {} <hashmap|btreemap|vec> N Q SEED", args[0]);
        std::process::exit(1);
    }

    let which = Some(args[1].clone());
    let n = map_bench::config::parse_arg::<usize>(&args[2], "N");
    let q = map_bench::config::parse_arg::<usize>(&args[3], "Q");
    let seed = map_bench::config::parse_arg::<u64>(&args[4], "SEED");
    map_bench::config::validate_inputs(n, q);

    let cfg = Config::new(n, q, seed);
    dispatch_or_run_all(which, cfg);
}
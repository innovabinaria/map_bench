//! CLI orchestration: dispatch a single benchmark or run all three.

use crate::{
    benches::{btreemap::bench_btreemap, hashmap::bench_hashmap, vec_sorted::bench_vec_sorted},
    config::Config,
};

/// Dispatch to a single benchmark (if `which` is Some) or run all three (if None).
pub fn dispatch_or_run_all(which: Option<String>, cfg: Config) {
    match which.as_deref() {
        Some("hashmap") => bench_hashmap(cfg),
        Some("btreemap") => bench_btreemap(cfg),
        Some("vec") => bench_vec_sorted(cfg),
        Some(other) => {
            eprintln!("Unknown type: '{}'. Use: hashmap | btreemap | vec", other);
            std::process::exit(1);
        }
        None => run_all(cfg),
    }
}

/// Run all three benchmarks in isolated subprocesses (RSS isolation).
pub fn run_all(cfg: Config) {
    let exe = std::env::current_exe().expect("Could not locate current executable");
    println!("➤ Config: N={}, Q={}, SEED={}", cfg.n, cfg.q, cfg.seed);

    for which in &["hashmap", "btreemap", "vec"] {
        println!("\n=== Benchmark: {} ===", which);
        let status = std::process::Command::new(&exe)
            .arg(which)
            .arg(cfg.n.to_string())
            .arg(cfg.q.to_string())
            .arg(cfg.seed.to_string())
            .status()
            .expect("Subprocess failed");
        if !status.success() {
            eprintln!("Subprocess '{}' exited with error.", which);
        }
    }
}

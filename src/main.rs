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




// //! Simple, reproducible benchmark comparing HashMap, BTreeMap and a sorted Vec.
// //!
// //! CLI compatibility preserved:
// //! - No args: runs all three benchmarks with N=1_000_000, Q=200_000, SEED=42.
// //! - With args: `map_bench <hashmap|btreemap|vec> N Q SEED`.
// //!
// //! Design notes:
// //! - Keys and indices are pre-generated to avoid RNG cost inside measured sections.
// //! - Process RSS is sampled during inserts to estimate peak memory.
// //! - No magic numbers: documented constants.
// //! - Clean code: small helpers, expressive names, clear errors.

// use rand::{rngs::StdRng, Rng, SeedableRng};
// use std::{
//     collections::{BTreeMap, HashMap},
//     env,
//     hint::black_box,
//     time::Instant,
// };
// use sysinfo::{Pid, ProcessesToUpdate, System};

// /* ============================ Configuration ============================ */

// const DEFAULT_N: usize = 1_000_000;
// const DEFAULT_Q: usize = 200_000;
// const DEFAULT_SEED: u64 = 42;

// const MIN_SAMPLE_STEP: usize = 10_000;

// // Cosmetic masks to avoid DCE; do not affect logic.
// const HM_MASK: u64 = 0xDEAD_BEEF;
// const BT_MASK: u64 = 0xC0FF_EE00;

// // Salts with uniform grouping to satisfy Clippy.
// const QUERY_SEED_SALT: u64 = 0x000A_11CE;
// const DELETE_SEED_SALT: u64 = 0x0BAD_5EED;

// #[derive(Clone, Copy, Debug)]
// struct Config {
//     /// Total inserts (collection size).
//     n: usize,
//     /// Total lookups after build.
//     q: usize,
//     /// PRNG seed (fully reproducible).
//     seed: u64,
//     /// Memory sampling interval during inserts (>= MIN_SAMPLE_STEP).
//     step: usize,
// }

// impl Config {
//     fn new(n: usize, q: usize, seed: u64) -> Self {
//         let step = std::cmp::max(MIN_SAMPLE_STEP, n.saturating_div(50));
//         Self { n, q, seed, step }
//     }
//     fn default_run_all() -> Self {
//         Self::new(DEFAULT_N, DEFAULT_Q, DEFAULT_SEED)
//     }
// }

// /* ============================ CLI / Entry ============================ */

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     if args.len() == 1 {
//         let cfg = Config::default_run_all();
//         run_all(cfg);
//         return;
//     }

//     if args.len() < 5 {
//         eprintln!("Usage: {} <hashmap|btreemap|vec> N Q SEED", args[0]);
//         std::process::exit(1);
//     }

//     let which = args[1].as_str();
//     let n = parse_arg::<usize>(&args[2], "N");
//     let q = parse_arg::<usize>(&args[3], "Q");
//     let seed = parse_arg::<u64>(&args[4], "SEED");
//     validate_inputs(n, q);

//     let cfg = Config::new(n, q, seed);

//     match which {
//         "hashmap" => bench_hashmap(cfg),
//         "btreemap" => bench_btreemap(cfg),
//         "vec" => bench_vec_sorted(cfg),
//         _ => {
//             eprintln!("Unknown type: '{}'. Use: hashmap | btreemap | vec", which);
//             std::process::exit(1);
//         }
//     }
// }

// /// Parse an argument with a clear error message.
// fn parse_arg<T: std::str::FromStr>(raw: &str, name: &str) -> T {
//     raw.parse::<T>()
//         .unwrap_or_else(|_| panic!("Invalid {}: '{}'", name, raw))
// }

// /// Validate N and Q to avoid pathological cases.
// fn validate_inputs(n: usize, q: usize) {
//     if n == 0 {
//         eprintln!("N must be > 0");
//         std::process::exit(1);
//     }
//     if q == 0 {
//         eprintln!("Q must be > 0");
//         std::process::exit(1);
//     }
// }

// /// Run all three benchmarks in separate subprocesses (isolates RSS).
// fn run_all(cfg: Config) {
//     let exe = std::env::current_exe().expect("Could not locate current executable");
//     println!("➤ Config: N={}, Q={}, SEED={}", cfg.n, cfg.q, cfg.seed);

//     for which in &["hashmap", "btreemap", "vec"] {
//         println!("\n=== Benchmark: {} ===", which);
//         let status = std::process::Command::new(&exe)
//             .arg(which)
//             .arg(cfg.n.to_string())
//             .arg(cfg.q.to_string())
//             .arg(cfg.seed.to_string())
//             .status()
//             .expect("Subprocess failed");
//         if !status.success() {
//             eprintln!("Subprocess '{}' exited with error.", which);
//         }
//     }
// }

// /* ============================ Memory utilities ============================ */

// #[inline]
// fn bytes_to_mib(bytes: u128) -> u64 {
//     (bytes / (1024u128 * 1024)) as u64
// }

// /// Process RSS sampler.
// struct MemSampler {
//     sys: System,
//     pid: Pid,
//     peak_mib: u64,
// }

// impl MemSampler {
//     fn new_current() -> Self {
//         let mut s = Self {
//             sys: System::new(),
//             pid: Pid::from_u32(std::process::id()),
//             peak_mib: 0,
//         };
//         s.peak_mib = s.current_mib();
//         s
//     }

//     /// Refresh only this process and return RSS in MiB.
//     fn current_mib(&mut self) -> u64 {
//         self.sys
//             .refresh_processes(ProcessesToUpdate::Some(&[self.pid]), true);
//         if let Some(p) = self.sys.process(self.pid) {
//             bytes_to_mib(p.memory() as u128) // sysinfo 0.37.x: bytes
//         } else {
//             0
//         }
//     }

//     /// Conditional sampling during insert loop; updates peak if needed.
//     fn maybe_sample(&mut self, i: usize, step: usize) {
//         if i.is_multiple_of(step) {
//             self.update_peak();
//         }
//     }

//     /// Update the peak based on current reading.
//     fn update_peak(&mut self) {
//         let now = self.current_mib();
//         if now > self.peak_mib {
//             self.peak_mib = now;
//         }
//     }

//     /// Peak observed so far.
//     fn peak(&self) -> u64 {
//         self.peak_mib
//     }
// }

// /* ============================ Data generation ============================ */

// /// Generate `n` deterministic u64 keys (based on `seed`).
// fn gen_keys(n: usize, seed: u64) -> Vec<u64> {
//     let mut rng = StdRng::seed_from_u64(seed);
//     (0..n).map(|_| rng.random::<u64>()).collect()
// }

// /// Generate `len` random indices in `[0, bound)`.
// fn gen_indices(len: usize, bound: usize, seed: u64) -> Vec<usize> {
//     let mut rng = StdRng::seed_from_u64(seed);
//     (0..len).map(|_| rng.random_range(0..bound)).collect()
// }

// /* ============================ Timing helper ============================ */

// #[inline]
// fn time_ms<F: FnOnce()>(f: F) -> u128 {
//     let t0 = Instant::now();
//     f();
//     t0.elapsed().as_millis()
// }

// /* ============================ Benchmarks ============================ */

// fn bench_hashmap(cfg: Config) {
//     let keys = gen_keys(cfg.n, cfg.seed);
//     let q_idx = gen_indices(cfg.q, cfg.n, cfg.seed ^ QUERY_SEED_SALT);
//     let d_idx = gen_indices(cfg.n / 2, cfg.n, cfg.seed ^ DELETE_SEED_SALT);

//     let mut mem = MemSampler::new_current();

//     // Insert
//     let mut map: HashMap<u64, u64> = HashMap::with_capacity(cfg.n);
//     let insert_ms = time_ms(|| {
//         for (i, &k) in keys.iter().enumerate() {
//             map.insert(k, k ^ HM_MASK);
//             mem.maybe_sample(i, cfg.step);
//         }
//     });
//     let mem_after_insert = mem.current_mib();
//     mem.update_peak();

//     // Queries
//     let mut found = 0u64;
//     let query_ms = time_ms(|| {
//         for &idx in &q_idx {
//             let k = keys[idx];
//             if let Some(v) = map.get(&k) {
//                 found ^= black_box(*v);
//             }
//         }
//     });

//     // Deletes (~ N/2)
//     let remove_ms = time_ms(|| {
//         for &idx in &d_idx {
//             let k = keys[idx];
//             let _ = map.remove(&k);
//         }
//     });

//     black_box(found);
//     black_box(&map);

//     println!("Insert: {} ms", insert_ms);
//     println!("Query:  {} ms", query_ms);
//     println!("Remove: {} ms", remove_ms);
//     println!("Mem after insert: {} MiB", mem_after_insert);
//     println!("Mem peak:         {} MiB", mem.peak());
// }

// fn bench_btreemap(cfg: Config) {
//     let keys = gen_keys(cfg.n, cfg.seed);
//     let q_idx = gen_indices(cfg.q, cfg.n, cfg.seed ^ QUERY_SEED_SALT);
//     let d_idx = gen_indices(cfg.n / 2, cfg.n, cfg.seed ^ DELETE_SEED_SALT);

//     let mut mem = MemSampler::new_current();

//     // Insert
//     let mut map: BTreeMap<u64, u64> = BTreeMap::new();
//     let insert_ms = time_ms(|| {
//         for (i, &k) in keys.iter().enumerate() {
//             map.insert(k, k ^ BT_MASK);
//             mem.maybe_sample(i, cfg.step);
//         }
//     });
//     let mem_after_insert = mem.current_mib();
//     mem.update_peak();

//     // Queries
//     let mut found = 0u64;
//     let query_ms = time_ms(|| {
//         for &idx in &q_idx {
//             let k = keys[idx];
//             if let Some(v) = map.get(&k) {
//                 found ^= black_box(*v);
//             }
//         }
//     });

//     // Deletes (~ N/2)
//     let remove_ms = time_ms(|| {
//         for &idx in &d_idx {
//             let k = keys[idx];
//             let _ = map.remove(&k);
//         }
//     });

//     black_box(found);
//     black_box(&map);

//     println!("Insert: {} ms", insert_ms);
//     println!("Query:  {} ms", query_ms);
//     println!("Remove: {} ms", remove_ms);
//     println!("Mem after insert: {} MiB", mem_after_insert);
//     println!("Mem peak:         {} MiB", mem.peak());
// }

// fn bench_vec_sorted(cfg: Config) {
//     let keys = gen_keys(cfg.n, cfg.seed);
//     let q_idx = gen_indices(cfg.q, cfg.n, cfg.seed ^ QUERY_SEED_SALT);
//     let d_idx = gen_indices(cfg.n / 2, cfg.n, cfg.seed ^ DELETE_SEED_SALT);

//     let mut mem = MemSampler::new_current();

//     // Build = push + sort to enable binary_search
//     let mut v: Vec<u64> = Vec::with_capacity(cfg.n);
//     let insert_ms = time_ms(|| {
//         for (i, &k) in keys.iter().enumerate() {
//             v.push(k);
//             mem.maybe_sample(i, cfg.step);
//         }
//         v.sort_unstable();
//     });
//     let mem_after_insert = mem.current_mib();
//     mem.update_peak();

//     // Queries
//     let mut found = 0u64;
//     let query_ms = time_ms(|| {
//         for &idx in &q_idx {
//             let k = keys[idx];
//             if let Ok(pos) = v.binary_search(&k) {
//                 found ^= black_box(v[pos]);
//             }
//         }
//     });

//     // Deletes (~ N/2) — O(n) per remove to keep order
//     let remove_ms = time_ms(|| {
//         for &idx in &d_idx {
//             let k = keys[idx];
//             if let Ok(pos) = v.binary_search(&k) {
//                 v.remove(pos);
//             }
//         }
//     });

//     black_box(found);
//     black_box(&v);

//     println!("Insert(+sort):    {} ms", insert_ms);
//     println!("Query(binsearch): {} ms", query_ms);
//     println!("Remove:           {} ms", remove_ms);
//     println!("Mem after insert: {} MiB", mem_after_insert);
//     println!("Mem peak:         {} MiB", mem.peak());
// }


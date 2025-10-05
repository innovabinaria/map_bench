//! Sorted Vec benchmark (binary_search for lookups; remove keeps order and is O(n)).

use std::hint::black_box;

use crate::{
    config::{Config, DELETE_SEED_SALT, QUERY_SEED_SALT},
    data::{gen_indices, gen_keys},
    mem::MemSampler,
    timing::time_ms,
};

pub fn bench_vec_sorted(cfg: Config) {
    let keys = gen_keys(cfg.n, cfg.seed);
    let q_idx = gen_indices(cfg.q, cfg.n, cfg.seed ^ QUERY_SEED_SALT);
    let d_idx = gen_indices(cfg.n / 2, cfg.n, cfg.seed ^ DELETE_SEED_SALT);

    let mut mem = MemSampler::new_current();

    // Build = push + sort to enable binary_search
    let mut v: Vec<u64> = Vec::with_capacity(cfg.n);
    let insert_ms = time_ms(|| {
        for (i, &k) in keys.iter().enumerate() {
            v.push(k);
            mem.maybe_sample(i, cfg.step);
        }
        v.sort_unstable();
    });
    let mem_after_insert = mem.current_mib();
    mem.update_peak();

    // Queries
    let mut found = 0u64;
    let query_ms = time_ms(|| {
        for &idx in &q_idx {
            let k = keys[idx];
            if let Ok(pos) = v.binary_search(&k) {
                found ^= black_box(v[pos]);
            }
        }
    });

    // Deletes (~ N/2) — O(n) per remove to keep order
    let remove_ms = time_ms(|| {
        for &idx in &d_idx {
            let k = keys[idx];
            if let Ok(pos) = v.binary_search(&k) {
                v.remove(pos);
            }
        }
    });

    black_box(found);
    black_box(&v);

    println!("Insert(+sort):    {} ms", insert_ms);
    println!("Query(binsearch): {} ms", query_ms);
    println!("Remove:           {} ms", remove_ms);
    println!("Mem after insert: {} MiB", mem_after_insert);
    println!("Mem peak:         {} MiB", mem.peak());
}

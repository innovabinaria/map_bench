//! HashMap benchmark.

use std::{collections::HashMap, hint::black_box};

use crate::{
    config::{Config, HM_MASK, DELETE_SEED_SALT, QUERY_SEED_SALT},
    data::{gen_indices, gen_keys},
    mem::MemSampler,
    timing::time_ms,
};

pub fn bench_hashmap(cfg: Config) {
    let keys = gen_keys(cfg.n, cfg.seed);
    let q_idx = gen_indices(cfg.q, cfg.n, cfg.seed ^ QUERY_SEED_SALT);
    let d_idx = gen_indices(cfg.n / 2, cfg.n, cfg.seed ^ DELETE_SEED_SALT);

    let mut mem = MemSampler::new_current();

    // Insert
    let mut map: HashMap<u64, u64> = HashMap::with_capacity(cfg.n);
    let insert_ms = time_ms(|| {
        for (i, &k) in keys.iter().enumerate() {
            map.insert(k, k ^ HM_MASK);
            mem.maybe_sample(i, cfg.step);
        }
    });
    let mem_after_insert = mem.current_mib();
    mem.update_peak();

    // Queries
    let mut found = 0u64;
    let query_ms = time_ms(|| {
        for &idx in &q_idx {
            let k = keys[idx];
            if let Some(v) = map.get(&k) {
                found ^= black_box(*v);
            }
        }
    });

    // Deletes (~ N/2)
    let remove_ms = time_ms(|| {
        for &idx in &d_idx {
            let k = keys[idx];
            let _ = map.remove(&k);
        }
    });

    black_box(found);
    black_box(&map);

    println!("Insert: {} ms", insert_ms);
    println!("Query:  {} ms", query_ms);
    println!("Remove: {} ms", remove_ms);
    println!("Mem after insert: {} MiB", mem_after_insert);
    println!("Mem peak:         {} MiB", mem.peak());
}

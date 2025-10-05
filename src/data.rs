//! Deterministic data generation (keys and indices).

use rand::{rngs::StdRng, Rng, SeedableRng};

/// Generate `n` deterministic u64 keys (based on `seed`).
pub fn gen_keys(n: usize, seed: u64) -> Vec<u64> {
    let mut rng = StdRng::seed_from_u64(seed);
    (0..n).map(|_| rng.random::<u64>()).collect()
}

/// Generate `len` random indices in `[0, bound)`.
pub fn gen_indices(len: usize, bound: usize, seed: u64) -> Vec<usize> {
    let mut rng = StdRng::seed_from_u64(seed);
    (0..len).map(|_| rng.random_range(0..bound)).collect()
}

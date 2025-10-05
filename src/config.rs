//! Configuration and CLI helpers.

/// Default full-run parameters.
pub const DEFAULT_N: usize = 1_000_000;
pub const DEFAULT_Q: usize = 200_000;
pub const DEFAULT_SEED: u64 = 42;

/// Minimum insert step between memory samples.
pub const MIN_SAMPLE_STEP: usize = 10_000;

/// Cosmetic masks (avoid DCE).
pub const HM_MASK: u64 = 0xDEAD_BEEF;
pub const BT_MASK: u64 = 0xC0FF_EE00;

/// Seed salts (uniform grouping to satisfy Clippy).
pub const QUERY_SEED_SALT: u64 = 0x000A_11CE;
pub const DELETE_SEED_SALT: u64 = 0x0BAD_5EED;

#[derive(Clone, Copy, Debug)]
pub struct Config {
    /// Total inserts (collection size).
    pub n: usize,
    /// Total lookups.
    pub q: usize,
    /// PRNG seed (reproducible).
    pub seed: u64,
    /// Memory sampling interval during inserts.
    pub step: usize,
}

impl Config {
    pub fn new(n: usize, q: usize, seed: u64) -> Self {
        let step = core::cmp::max(MIN_SAMPLE_STEP, n.saturating_div(50));
        Self { n, q, seed, step }
    }

    pub fn default_run_all() -> Self {
        Self::new(DEFAULT_N, DEFAULT_Q, DEFAULT_SEED)
    }
}

/// Parse an argument with a clear error message.
pub fn parse_arg<T: core::str::FromStr>(raw: &str, name: &str) -> T {
    raw.parse::<T>()
        .unwrap_or_else(|_| panic!("Invalid {}: '{}'", name, raw))
}

/// Validate inputs to avoid pathological cases.
pub fn validate_inputs(n: usize, q: usize) {
    if n == 0 {
        eprintln!("N must be > 0");
        std::process::exit(1);
    }
    if q == 0 {
        eprintln!("Q must be > 0");
        std::process::exit(1);
    }
}

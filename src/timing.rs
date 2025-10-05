//! Small timing utility.

use std::time::Instant;

/// Measure `f` execution and return its duration in milliseconds.
#[inline]
pub fn time_ms<F: FnOnce()>(f: F) -> u128 {
    let t0 = Instant::now();
    f();
    t0.elapsed().as_millis()
}

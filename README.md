# map_bench

A small, reproducible, and **learning-oriented** benchmark to compare **time** and **memory** across three Rust data structures:

* `HashMap<u64, u64>`
* `BTreeMap<u64, u64>`
* Sorted `Vec<u64>` (queries via `binary_search`)

For each structure, the program builds with `N` items, runs `Q` lookups, and deletes ~`N/2` random keys. During build, it samples the process **RSS (resident set size)** to estimate **peak memory**.

---

## ✨ What exactly does it measure?

For each structure:

* **Insert** — wall-clock time to build the collection with `N` `(key, value)` pairs.
* **Query** — wall-clock time to perform `Q` exact lookups.
* **Remove** — wall-clock time to delete ~`N/2` random keys.
* **Mem after insert** — process RSS (MiB) right after the build phase.
* **Mem peak** — maximum RSS observed during build (sampled periodically).

> Keys and random indices are **pre-generated** from a fixed seed so the timings reflect the data structure itself, not RNG overhead.

---

## 🧰 Tech stack

* **Rust** (edition **2024**)
* Crates:

  * [`rand` 0.9.x](https://crates.io/crates/rand) — deterministic data generation (PRNG).
  * [`sysinfo` 0.37.x](https://crates.io/crates/sysinfo) — process RSS readings.

---

## 📁 Project layout

```

src/
├─ main.rs                 # CLI: no args => run all; with args => one benchmark
├─ lib.rs                  # Library crate entry
├─ config.rs               # Config (N, Q, SEED, step) + CLI helpers
├─ cli.rs                  # Orchestration: run_all / dispatch
├─ mem.rs                  # Memory sampling (sysinfo)
├─ data.rs                 # Deterministic key/index generation
├─ timing.rs               # Timing helper (ms)
└─ benches/
   ├─ hashmap.rs           # HashMap benchmark
   ├─ btreemap.rs          # BTreeMap benchmark
   └─ vec_sorted.rs        # Sorted Vec + binary_search benchmark
```

---

## ✅ Requirements

* **Rust** (latest stable recommended)
* A platform supported by `sysinfo` (Linux, macOS, Windows)

Minimal `Cargo.toml`:

```toml
[package]
name = "map_bench"
version = "0.1.0"
edition = "2024"

[dependencies]
rand = "0.9.2"
sysinfo = "0.37.2"
```

---

## ▶️ Build & Run

```bash
# 1) Build in release
cargo build --release

# 2) Run: no arguments runs all three benchmarks with defaults
./target/release/map_bench

# 3) Run a specific benchmark:
#    map_bench <hashmap|btreemap|vec> N Q SEED
./target/release/map_bench hashmap 1000000 200000 42
./target/release/map_bench btreemap 1000000 200000 42
./target/release/map_bench vec      1000000 200000 42
```

**Sample output:**

```
➤ Config: N=1000000, Q=200000, SEED=42

=== Benchmark: hashmap ===
Insert: 1644 ms
Query:  101 ms
Remove: 244 ms
Mem after insert: 48 MiB
Mem peak:         48 MiB
```

---

## 📖 Interpreting the results

* **Sorted Vec**

  * Pros: **compact** memory; `binary_search` often **very fast** (contiguous, cache-friendly).
  * Cons: `remove(pos)` **keeps order**, so it shifts elements ⇒ **O(n)** per deletion. With many deletes, time blows up.

* **HashMap**

  * Pros: inserts/lookups **amortized O(1)**; fast deletes.
  * Cons: higher **overhead** (buckets/metadata) than a dense Vec.

* **BTreeMap**

  * Pros: **key order** and efficient range/ordered iteration; stable performance.
  * Cons: operations are **O(log n)**; node/pointer overhead.

---

## 🔍 Accuracy & reproducibility

* Keys and indices come from a deterministic PRNG (`StdRng` + `SEED`) → repeatable runs on the same hardware/OS.
* `sysinfo` (v0.37) reports `process.memory()` in **bytes**; the program converts to **MiB** for display.
* Timings are **wall-clock** (they include OS scheduling, caches, allocator effects, etc.).

---

* **Memory looks absurdly large (tens of GB)**
  That usually happens if bytes are misinterpreted as KiB/MiB. This repo converts bytes → MiB correctly.

* **Unstable timings**
  Close heavy apps, use `--release`, run multiple times and average. Increasing `N`/`Q` improves signal/noise.

---


## 🌐 Author

Victor Aguayo — Technology Architect & Rust Enthusiast

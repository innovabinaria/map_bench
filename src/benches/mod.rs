// src/benches/mod.rs
pub mod hashmap;
pub mod btreemap;
pub mod vec_sorted;

// (opcional) re-exporta las funciones para imports más cortos:
pub use btreemap::bench_btreemap;
pub use hashmap::bench_hashmap;
pub use vec_sorted::bench_vec_sorted;

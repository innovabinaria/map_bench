pub mod hashmap;
pub mod btreemap;
pub mod vec_sorted;

pub use btreemap::bench_btreemap;
pub use hashmap::bench_hashmap;
pub use vec_sorted::bench_vec_sorted;


// Reference:
//      https://wangchujiang.com/rust-cn-document-for-docker/std/core/hash/struct.BuildHasherDefault.html#
//      https://tinyzzh.github.io/rust/2023/04/10/rust_lang_tutorial_132_Trait_Hash.html

use axhal::misc::random;
use core::hash::Hash;
use hashbrown::HashMap;
use foldhash::fast::FixedState;

// Hasher: calculate hash value from input bytes
// pub struct MyHasher {
//     seed: u64,  // record the seed value, don't change after initialization
//     hash: u64,
// }

// impl MyHasher {
//     pub fn new(seed: u64) -> Self {
//         Self {
//             seed,
//             hash: seed,
//         }
//     }
// }

// impl Hasher for MyHasher {
//     fn write(&mut self, bytes: &[u8]) {
//         for byte in bytes {
//             self.hash = self.hash.wrapping_mul(31).wrapping_add(*byte as u64);
//         }
//     }
//     fn finish(&self) -> u64 {
//         self.hash
//     }
// }

// HasherBuilder: create Hasher instance
// struct MyHasherBuilder;

// impl BuildHasher for MyHasherBuilder {
//     type Hasher = MyHasher;
//     fn build_hasher(&self) -> MyHasher {
//         // Note: Hasher trait's finish function returns u64, convert it to u64 for simplicity.
//         MyHasher::new(random() as u64)
//     }
// }

// MyHashMap: hashmap using custom hasher
pub struct MyHashMap<K, V> {
    map: HashMap<K, V, FixedState>,
}

impl<K, V> MyHashMap<K, V> {
    pub fn new() -> Self
    where
        K: Hash + Eq,
    {
        Self {
            map: HashMap::with_hasher(FixedState::with_seed(random() as _)),
            // map: HashMap::with_hasher(MyHasherBuilder),
        }
    }

    pub fn insert(&mut self, key: K, value: V)
    where 
        K: Hash + Eq,
    {
        self.map.insert(key, value);
    }

    pub fn get(&self, key: &K) -> Option<&V> 
    where 
        K: Hash + Eq,
    {
        self.map.get(key)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> 
    where 
        K: Hash + Eq,
    {
        self.map.remove(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> 
    where 
        K: Hash + Eq,
    {
        self.map.iter()
    }
}

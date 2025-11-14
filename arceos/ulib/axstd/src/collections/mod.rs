
// Note: alloc::collections doesn't provide HashMap.

pub mod hash;

pub use hash::map::MyHashMap as HashMap;

pub use alloc::collections::*;
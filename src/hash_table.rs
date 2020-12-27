//! Hash table
//!
//! An idea is very simple. We put values in the array, but not just by pushing them. We have some smart algorithm for pushing,
//! which requires having a way to compute the position of inserting value. The "way" is actually a hash function. Look at `HashTable::eval_index` method.
//! There is a crucial contract in current implementation: size and capacity of buckets container are equal, however the "real length", which is amount
//! of initialized (more preciously, non-zero or non-empty in our case) values differs from capacity.

use std::hash::Hash;

use buckets::Buckets;

const INITIAL_LEN: usize = 2;

pub(super) struct HashTable<K: Hash + Eq, V> {
    buckets: Buckets<K, V>,
}

impl<K: Hash + Eq, V> HashTable<K, V> {
    pub(super) fn new() -> Self {
        HashTable {
            buckets: Buckets::new(),
        }
    }

    pub(super) fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.needs_resize() {
            self.resize()
        }
        self.buckets.insert(key, value)
    }

    pub(super) fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    pub(super) fn get<'a>(&'a self, key: &'a K) -> Option<&'a V> {
        self.buckets.get(key)
    }

    pub(super) fn remove(&mut self, key: &K) -> Option<V> {
        self.buckets.remove(key)
    }

    pub(super) fn len(&self) -> usize {
        self.buckets.count_items()
    }

    fn needs_resize(&self) -> bool {
        let real_len = self.buckets.len();
        if real_len == 0 {
            return true;
        }
        // small occupancy reduces collision probability
        let occupancy = real_len / self.buckets.cap();
        occupancy > 7
    }

    // If we need to resize bucket vector to prevent collision,
    // then we have to move values from original bucket to the new one.
    // What's the purpose of the move? Why can't we just enlarge our vector, making
    // capacity n * 2? That's because indexing depends on the size of bucket vector,
    // so previous indexes become invalid.
    fn resize(&mut self) {
        let cap = match self.buckets.cap() {
            // "0" case will be called on the first insert
            0 => INITIAL_LEN,
            n => n * 2,
        };
        self.buckets.resize(cap)
    }
}

mod buckets {
    //! Hidden in mod just to control API

    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::mem::replace;

    // Not just V, but (K, V), because if we have faced collision and
    // there's more than 1 element in vector, then we need to some how recognize
    // desired value.
    //
    // Let's just use Vec<Vec> instead of Vec<LinkedList>.
    pub(super) struct Buckets<K: Hash + Eq, V>(Vec<Vec<(K, V)>>);

    impl<K: Hash + Eq, V> Buckets<K, V> {
        pub(super) fn new() -> Self {
            // Initializing like this `vec![Vec::new(), some_cap] needs K to implement Clone`,
            // but by doing current zero initialization we enlarge our trait bound.
            Self(Vec::new())
        }

        pub(super) fn remove(&mut self, key: &K) -> Option<V> {
            let i = self.get_index(&key);
            // Deleting if entry exists by finding its index in `self.0[i]` bucket and returning value
            if let Some(pos) = self.get_pos_in_bucket(i, key) {
                let (_, v) = self.0[i].swap_remove(pos);
                return Some(v);
            }
            None
        }

        pub(super) fn insert(&mut self, key: K, value: V) -> Option<V> {
            let i = self.get_index(&key);
            // Checking if entry exists by finding its index in `self.0[i]` bucket and returning value
            if let Some(pos) = self.get_pos_in_bucket(i, &key) {
                let (_, v) = replace(&mut self.0[i][pos], (key, value));
                return Some(v);
            }
            // Otherwise there isn't such key and we add a new one
            self.0[i].push((key, value));
            None
        }

        fn get_pos_in_bucket(&self, bucket: usize, key: &K) -> Option<usize> {
            self.0
                .get(bucket)
                .map(|bucket| bucket.iter().position(|(k, _)| k == key))
                .flatten()
        }

        pub(super) fn get<'a>(&'a self, key: &'a K) -> Option<&'a V> {
            let i = self.get_index(key);
            self.0
                .get(i)
                .map(|bucket| bucket.iter().find(|(k, _)| k == key))
                .flatten()
                .map(|(_, v)| v)
        }

        pub(super) fn cap(&self) -> usize {
            self.0.capacity()
        }

        // This is strange at first glance. But the fact that we initialize bucket with default values by `resize_with(cap, || Vec::new())` at `Self::resize`
        // means that we have `self.0.len() == self.0.cap()`. So the "real len" is an amount of non-empty buckets in `self.0` vector.
        pub(super) fn len(&self) -> usize {
            self.0.iter().filter(|l| !l.is_empty()).count()
        }

        // Amount of entries. Note: not `self.0.len()`, which is the same as cap in current implementation,
        // nor the `Self::count_non_empty()`
        pub(super) fn count_items(&self) -> usize {
            self.0.iter().fold(0, |acc, l| acc + l.len())
        }

        pub(super) fn resize(&mut self, cap: usize) {
            let mut new_buckets = Vec::with_capacity(cap);
            new_buckets.resize_with(cap, || Vec::new());

            let old_buckets = replace(&mut self.0, new_buckets);
            self.update_old_values(old_buckets);
        }

        fn update_old_values(&mut self, old_buckets: Vec<Vec<(K, V)>>) {
            // Fill in with old entries, but with new indexes.
            for bucket in old_buckets {
                let (k, _) = bucket.get(0).expect("existing bucket can't be empty");
                let new_index = self.get_index(k);
                self.0[new_index] = bucket;
            }
        }

        // Just a standard way of getting index for the key
        fn get_index(&self, key: &K) -> usize {
            let mut h = DefaultHasher::new();
            key.hash(&mut h);
            let hash = h.finish() as usize;
            // This is very important! `hash mod array_size`. In current implementation array_size == len == cap.
            hash % self.0.len()
        }
    }
}

#[test]
fn simple() {
    let mut ht = HashTable::new();
    ht.insert(1, "23");
    assert_eq!(ht.len(), 1);
    assert!(!ht.contains_key(&123));
    assert!(ht.get(&1).is_some());
    let prev = ht.insert(1, "13");
    assert!(prev.is_some());
    assert_eq!(prev, Some("23"));
    ht.insert(2, "23");
    ht.insert(3, "23");
    ht.insert(4, "23");
    assert_eq!(ht.len(), 4);
    ht.remove(&1);
    assert!(!ht.contains_key(&1));
}

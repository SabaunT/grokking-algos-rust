//! Hash table
//!
//! An idea is very simple. We put values in the array, but not just by pushing them. We have some smart algorithm for pushing,
//! which requires having a way to compute the position of inserting value. The "way" is actually a hash function. Look at `HashTable::eval_index` method.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem::replace;

const INITIAL_LEN: usize = 2;

pub(super) struct HashTable<K, V>
where
    K: Hash + Eq,
{
    // Not just V, but (K, V), because if we have faced collision and
    // there's more than 1 element in vector, then we need to some how recognize
    // desired value.
    //
    // Let's just use Vec<Vec> instead of Vec<LinkedList>.
    buckets: Vec<Vec<(K, V)>>,
}

impl<K, V> HashTable<K, V>
where
    K: Hash + Eq,
{
    pub(super) fn new() -> Self {
        // Initialize with len equal to cap
        // Initializing like this `vec![Vec::new(), some_cap] needs K to implement Clone`,
        // but by doing current zero initialization and `self.resize()` on first insert we enlarge our trait bound.
        HashTable {
            buckets: Vec::new(),
        }
    }

    pub(super) fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.needs_resize() {
            self.resize()
        }

        let index = self.eval_index(&key);
        // Checking if value exists by finding its index in `self.bucket[index]` and returning it
        if let Some(pos) = self.buckets[index].iter().position(|(k, _)| k == &key) {
            let (_, v) = replace(&mut self.buckets[index][pos], (key, value));
            return Some(v);
        }
        self.buckets[index].push((key, value));

        None
    }

    pub(super) fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    pub(super) fn get(&self, key: &K) -> Option<&V> {
        let index = self.eval_index(key);
        self.buckets
            .get(index)
            .map(|l| l.iter().find(|(k, _)| k == key).map(|(_, v)| v))
            .flatten()
    }

    pub(super) fn remove(&mut self, key: &K) -> Option<V> {
        if self.get(key).is_some() {
            let index = self.eval_index(&key);
            let key_index = self.buckets[index].iter().position(|(k, _)| k == key)?;
            let (_, v) = self.buckets[index].swap_remove(key_index);
            return Some(v);
        }
        None
    }

    // Amount of entries. Note: not `self.buckets.len()`, which is the same as cap in current implementation,
    // nor the `self.non_empty_buckets()`
    pub(super) fn len(&self) -> usize {
        self.buckets.iter().fold(0, |acc, l| acc + l.len())
    }

    fn needs_resize(&self) -> bool {
        let non_empty_buckets = self.non_empty_buckets();
        if non_empty_buckets == 0 {
            return true;
        }
        // small occupancy reduces collision probability
        let occupancy = non_empty_buckets / self.buckets.capacity();
        occupancy > 7
    }

    // This is strange at first glance. But the fact that we initialize bucket with default values by `resize_with(cap, || Vec::new())`
    // means that we have `self.bucket.len() == self.bucket.cap()`. So the "real len" is an amount of non-empty buckets in `self.bucket` vector.
    fn non_empty_buckets(&self) -> usize {
        self.buckets.iter().filter(|l| !l.is_empty()).count()
    }

    // If we need to resize bucket vector to prevent collision,
    // then we have to move values from original bucket to the new one.
    // What's the purpose of the move? Why can't we just enlarge our vector, making
    // capacity n * 2? That's because indexing depends on the size of bucket vector,
    // so previous indexes become invalid.
    fn resize(&mut self) {
        // todo занеси в evernote. Намного лучше, чем cap = abssd(); if cap == 0 и блабла
        let cap = match self.buckets.capacity() {
            // "0" case will be called on the first insert
            0 => INITIAL_LEN,
            n => n * 2,
        };
        let mut new_bucket = Vec::with_capacity(cap);
        new_bucket.resize_with(cap, || Vec::new());

        let old_bucket = replace(&mut self.buckets, new_bucket);

        // Fill in with old entries, but with new indexes.
        for list in old_bucket {
            let (k, _) = list.get(0).expect("existing list can't be empty");
            let new_index = self.eval_index(k);
            self.buckets[new_index] = list;
        }
    }

    // Just a standard way of getting index for the key
    fn eval_index(&self, key: &K) -> usize {
        let mut h = DefaultHasher::new();
        key.hash(&mut h);
        let hash = h.finish() as usize;
        // This is very important! `hash mod array_size`. In current implementation array_size == len == cap.
        hash % self.buckets.len()
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

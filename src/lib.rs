extern crate rand;

use std::mem;
use std::cmp::PartialEq;
use std::hash::{Hash, Hasher, SipHasher};
use std::borrow::Borrow;

use rand::Rng;

#[derive(Debug)]
pub struct HashTable<K: PartialEq + Hash, V> {
    entries: Vec<HashEntry<K, V>>,
    item_count: usize,
    hash_state: HashState,
}

/// Stores the randomly generated SipHash keys
#[derive(Debug)]
struct HashState {
    k0: u64,
    k1: u64,
}

/// All of the nodes that have a particular hash value
#[derive(Debug)]
struct HashEntry<K: PartialEq + Hash, V> {
    items: Vec<Node<K, V>>,
}

/// Each inserted item
#[derive(Debug)]
struct Node<K: PartialEq + Hash, V> {
    key: K,
    value: V,
}

impl<K, V> HashTable<K, V> where K: PartialEq + Hash {
    pub fn new() -> Self {
        HashTable::with_capacity(3)
    }

    pub fn with_capacity(entry_cap: usize) -> Self {
        HashTable {
            entries: Self::make_entries(entry_cap),
            item_count: 0,
            hash_state: HashState::new(),
        }
    }

    pub fn get<'a, Q>(&self, key: &'a Q) -> Option<&V> where Q: Hash + PartialEq, K: Borrow<Q> {
        let entry = self.find_entry(key);
        Self::find_node(key, entry).map(|node| &node.value)
    }

    pub fn get_mut<'a, Q>(&mut self, key: &'a Q) -> Option<&mut V> where Q: Hash + PartialEq, K: Borrow<Q> {
        let entry = self.find_entry_mut(key);
        Self::find_node_mut(key, entry).map(|node| &mut node.value)
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<(K, V)> {
        if self.fill_factor() >= 3.0 {
            self.grow();
        }

        {
            let entry = self.find_entry_mut(&key);
            if let Some(node) = Self::find_node_mut(&key, entry) {
                let key = mem::replace(&mut node.key, key);
                let value = mem::replace(&mut node.value, value);
                return Some((key, value));
            }

            entry.items.push(Node {
                key: key,
                value: value,
            });
        }
        self.item_count += 1;
        None
    }

    fn make_entries(cap: usize) -> Vec<HashEntry<K, V>> {
        (0..cap).map(|_| Default::default()).collect()
    }

    fn find_node<'a, Q>(key: &Q, entry: &'a HashEntry<K, V>) -> Option<&'a Node<K, V>>
        where Q: Hash + PartialEq, K: Borrow<Q> {
        
        entry.items.iter().filter(|node| node.key.borrow() == key).next()
    }

    fn find_entry<'a, Q>(&'a self, key: &Q) -> &'a HashEntry<K, V> where Q: Hash + PartialEq, K: Borrow<Q> {
        let mut hasher = self.hash_state.get_hasher();
        key.hash(&mut hasher);
        let hash = hasher.finish();
        let index = (hash % self.entries.len() as u64) as usize;

        &self.entries[index]
    }

    fn find_node_mut<'a, Q>(key: &Q, entry: &'a mut HashEntry<K, V>) -> Option<&'a mut Node<K, V>>
        where Q: Hash + PartialEq, K: Borrow<Q> {

        entry.items.iter_mut().filter(|node| node.key.borrow() == key).next()
    }

    fn find_entry_mut<'a, Q>(&'a mut self, key: &Q) -> &'a mut HashEntry<K, V> where Q: Hash + PartialEq, K: Borrow<Q> {
        let mut hasher = self.hash_state.get_hasher();
        key.hash(&mut hasher);
        let hash = hasher.finish();
        let index = (hash % self.entries.len() as u64) as usize;

        &mut self.entries[index]
    }

    fn fill_factor(&self) -> f32 {
        self.item_count as f32 / self.entries.len() as f32
    }

    fn grow(&mut self) {
        let new_size = self.entries.len() * 2;
        let old_entries = mem::replace(&mut self.entries, Self::make_entries(new_size));
        self.item_count = 0;

        for entry in old_entries {
            for item in entry.items {
                self.insert(item.key, item.value);
            }
        }
    }
}

impl HashState {
    fn new() -> Self {
        let r = rand::OsRng::new();
        let mut r = r.expect("Failed to create an RNG to seed the hash state");
        HashState {
            k0: r.gen(),
            k1: r.gen(),
        }
    }

    fn get_hasher(&self) -> SipHasher {
        SipHasher::new_with_keys(self.k0, self.k1)
    }
}

impl<K, V> Default for HashEntry<K, V> where K: PartialEq + Hash {
    fn default() -> Self {
        HashEntry {
            items: vec![],
        }
    }
}

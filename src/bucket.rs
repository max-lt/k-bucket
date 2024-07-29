use std::collections::BTreeMap;

use crate::Arbiter;
use crate::GetDirection;
use crate::GetDistance;
use crate::GetKey;
use crate::LeadingZeros;

/// K: max items in a bucket
pub struct Bucket<Key, Item: GetKey<Key>, const K: usize> {
    pub(crate) key: Key,
    pub(crate) buckets: BTreeMap<u8, Vec<Item>>,
    key_bits: u8,
}

/// Key: key struct
/// Item: value struct
/// K: max items in a bucket
impl<const K: usize, Key, Item> Bucket<Key, Item, K>
where
    Key: PartialEq + GetDirection + Clone + GetDistance + LeadingZeros + Ord + std::fmt::Debug,
    Item: GetKey<Key> + Arbiter + std::fmt::Debug,
{
    pub fn new(key: Key) -> Self {
        let key_bits = std::mem::size_of::<Key>() * 8;

        assert!(K > 0, "K must be greater than 0");
        assert!(key_bits > 0, "Key must have bits");
        assert!(key_bits <= 256, "Key must have less than 256 bits");

        let key_bits = key_bits as u8;

        Bucket {
            key,
            buckets: BTreeMap::new(),
            key_bits,
        }
    }

    pub fn put(&mut self, value: Item) {
        let item_key = value.get_key();

        let bucket_index = self.bucket_index(&item_key);

        let items = self.buckets.entry(bucket_index).or_insert(Vec::new());

        // Check if item already exists
        let item_index = items.iter().position(|item| item.get_key() == item_key);

        // Update item
        if let Some(item_index) = item_index {
            // items[item_index] = value;
            let incumbent = &items[item_index];
            let should_update = incumbent.arbitrate(&value);

            // If the selected item is the same as the incumbent, do nothing
            if !should_update {
                return;
            }

            // If the selected item is the new item, update
            items.remove(item_index);
            items.push(value);

            return;
        }

        // Bucket is not full, insert item
        if items.len() < K {
            items.push(value);
            return;
        }
    }

    pub fn get<'a>(&'a self, key: &Key) -> Option<&'a Item> {
        let bucket_index = self.bucket_index(key);

        let items = &self.buckets.get(&bucket_index)?;

        // Check if item exists
        items.iter().find(|item| item.get_key() == *key)
    }

    pub fn del(&mut self, key: &Key) -> Option<Item> {
        let bucket_index = self.bucket_index(key);

        let items = &mut self.buckets.get_mut(&bucket_index)?;

        // Check if item exists
        let item_index = items.iter().position(|item| item.get_key() == *key)?;

        Some(items.remove(item_index))
    }

    fn bucket_index(&self, key: &Key) -> u8 {
        let bucket_key = self.key.clone();

        let index = bucket_key.distance(key).leading_zeros() as u8;

        if index > self.key_bits {
            panic!("Index out of bounds");
        }

        if index < self.key_bits {
            index
        } else {
            self.key_bits - 1
        }
    }

    /// Count the number of items in the bucket
    pub fn count(&self) -> usize {
        self.buckets.values().map(|items| items.len()).sum()
    }

    /// Get the n closest items to the given key
    pub fn closest(&self, key: &Key, n: usize) -> Vec<&Item> {
        let mut result = self
            .buckets
            .values()
            .flat_map(|items| items.iter())
            .map(|item| (key.distance(&item.get_key()), item))
            .collect::<Vec<(Key, &Item)>>();

        result.sort_by(|(a, _), (b, _)| a.cmp(&b));

        result.truncate(n);

        result.into_iter().map(|(_, item)| item).collect()
    }
}

#[cfg(test)]
mod tests {}

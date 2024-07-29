use std::fmt::Debug;

use crate::Bucket;
use crate::GetKey;

impl<Key: Debug, Item: Debug + GetKey<Key>, const K: usize> Debug for Bucket<Key, Item, K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        let key_bits = std::mem::size_of::<Key>() * 8;

        result.push_str(&format!("Bucket({:?}, k={}, n={})\n", self.key, K, key_bits));

        for (i, items) in self.buckets.iter() {
            result.push_str(&format!("Bucket[{}]: {:?}\n", i, items));
        }

        write!(f, "{}", result)
    }
}

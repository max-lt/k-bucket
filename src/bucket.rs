
use log::debug;

use crate::GetKey;
use crate::HasBitAt;
use crate::Node;

/// K: max items in a bucket
pub struct Bucket<Key, Item, const K: usize> {
    pub (crate) root: Box<Node<Item>>,
    pub (crate) key: Key,
}

/// Key: key struct
/// Item: value struct
/// K: max items in a bucket
impl<const K: usize, Key: PartialEq + HasBitAt, Item: GetKey<Key>> Bucket<Key, Item, K> {
    pub fn new(key: Key) -> Self {
        Bucket {
            key,
            root: Box::new(Node::new(true)),
        }
    }

    pub fn put(&mut self, value: Item) {
        let mut bit_index = 0;
        let mut node = self.root.as_mut();

        let key = value.get_key();

        // Navigate to the first bucket node
        while node.items.is_none() {
            node = match key.has_bit_at(bit_index) {
                false => node.left.as_mut().unwrap(),
                true => node.right.as_mut().unwrap(),
            };

            bit_index += 1;
        }

        // Check if item already exists
        let item_index = match &node.items {
            Some(items) => items.iter().position(|item| item.get_key() == key),
            // This should never happen as we navigated to the bucket node earlier
            None => unreachable!("Node has no items"),
        };

        let items = node.items.as_mut().unwrap();

        // Update item
        if let Some(item_index) = item_index {
            items[item_index] = value;

            return;
        }

        // Bucket is not full, insert item
        if items.len() < K {
            items.push(value);
            return;
        }

        // We cannot split the node, remove the oldest item and insert the new one
        if !node.can_split {
            // items.remove(0);
            // items.push(value);
            return;
        }

        // Split node and distribute items
        debug!("Splitting bucket {}", bit_index);
        let mut left_items = Vec::new();
        let mut right_items = Vec::new();

        // Distribute items
        while let Some(item) = items.pop() {
            match item.get_key().has_bit_at(bit_index) {
                false => left_items.push(item),
                true => right_items.push(item),
            }
        }

        let is_right = self.key.has_bit_at(bit_index);
        debug!(
            "Splitting bucket {} left={} right={}",
            bit_index, !is_right, is_right
        );

        node.items = None;
        node.left = Some(Box::new(Node::new_with_items(left_items, !is_right)));
        node.right = Some(Box::new(Node::new_with_items(right_items, is_right)));
        node.can_split = false;
        // End of split - TODO move to node impl

        // Insert value by recursion
        self.put(value);
    }

    pub fn get<'a>(&'a self, key: &Key) -> Option<&'a Item> {
        let mut bit_index = 0;
        let mut node = self.root.as_ref();

        // Navigate to the first bucket node
        while node.items.is_none() {
            node = match key.has_bit_at(bit_index) {
                false => node.left.as_ref().unwrap(),
                true => node.right.as_ref().unwrap(),
            };

            bit_index += 1;
        }

        // Check if item exists
        let items = node.items.as_ref().unwrap();

       items.iter().find(|item| item.get_key() == *key)
    }  

    pub fn del(&mut self,  key: &Key) {
        let mut bit_index = 0;
        let mut node = self.root.as_mut();

        // Navigate to the first bucket node
        while node.items.is_none() {
            node = match key.has_bit_at(bit_index) {
                false => node.left.as_mut().unwrap(),
                true => node.right.as_mut().unwrap(),
            };

            bit_index += 1;
        }

        // Check if item exists
        let items = node.items.as_ref().unwrap();

        let item_index = items.iter().position(|item| item.get_key() == *key);

        if let Some(item_index) = item_index {
            let items = node.items.as_mut().unwrap();
            items.remove(item_index);
        }
    }
}

#[cfg(test)]
mod tests {
    
}

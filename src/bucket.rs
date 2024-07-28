use log::debug;

use crate::traits::Arbiter;
use crate::GetDirection;
use crate::GetDistance;
use crate::GetKey;
use crate::Node;

/// K: max items in a bucket
pub struct Bucket<Key, Item: GetKey<Key>, const K: usize> {
    pub(crate) root: Box<Node<Key, Item>>,
    pub(crate) key: Key,
}

/// Key: key struct
/// Item: value struct
/// K: max items in a bucket
impl<const K: usize, Key, Item> Bucket<Key, Item, K>
where
    Key: PartialEq + GetDirection + Clone + GetDistance + Ord,
    Item: GetKey<Key> + Arbiter,
{
    pub fn new(key: Key) -> Self {
        Bucket {
            key,
            root: Box::new(Node::new_root()),
        }
    }

    pub fn put(&mut self, value: Item) {
        let bucket_key = self.key.clone();

        let item_key = value.get_key();

        let (node, bit_index) = Node::get_node_mut(&mut self.root, &item_key);

        // Check if item already exists
        let item_index = match &node.items {
            Some(items) => items.iter().position(|item| item.get_key() == item_key),
            // This should never happen as we navigated to the bucket node earlier
            None => unreachable!("Node has no items"),
        };

        let items = node.items.as_mut().unwrap();

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

        // The bucket is full
        if !node.can_split {
            // TODO: emit event with the m items list and the
            // new item to let the user decide what to do,
            // for now we just ignore the new item and return
            return;
        }

        // Split node and distribute items
        debug!("Splitting bucket {}", bit_index);
        node.split(bit_index, bucket_key.direction(bit_index));

        // Insert value by recursion
        self.put(value);
    }

    pub fn get<'a>(&'a self, key: &Key) -> Option<&'a Item> {
        let (node, _) = Node::get_node(&self.root, key);

        // Check if item exists
        let items = node.items.as_ref().unwrap();

        items.iter().find(|item| item.get_key() == *key)
    }

    pub fn del(&mut self, key: &Key) -> Option<Item> {
        let (node, _) = Node::get_node_mut(&mut self.root, key);

        // Check if item exists
        let items = node.items.as_mut().unwrap();
        let item_index = items.iter().position(|item| item.get_key() == *key)?;

        Some(items.remove(item_index))
    }

    /// Count the number of items in the bucket
    pub fn count(&self) -> usize {
        self.root.count()
    }

    /// Get the n closest items to the given key
    pub fn closest(&self, key: &Key, n: usize) -> Vec<&Item> {
        let mut result = Vec::new();

        let mut nodes = vec![&self.root];

        while !nodes.is_empty() {
            let node = nodes.pop().unwrap();

            match &node.items {
                Some(items) => result.extend(items.iter()),
                None => {
                    nodes.push(&node.left.as_ref().unwrap());
                    nodes.push(&node.right.as_ref().unwrap());
                }
            }
        }

        let mut result = result
            .iter()
            .map(|item| (key.distance(&item.get_key()), *item))
            .collect::<Vec<(Key, &Item)>>();

        result.sort_by(|(a, _), (b, _)| a.cmp(&b));

        result.into_iter().take(n).map(|(_, item)| item).collect()
    }
}

#[cfg(test)]
mod tests {}

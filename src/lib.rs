use log::debug;

pub type Buf<const N: usize> = [u8; N];

pub trait Distance<const N: usize> {
    fn distance(&self, other: &Buf<N>) -> Buf<N>;
}

impl<const N: usize> Distance<N> for Buf<N> {
    /// XOR distance between two keys of size N
    fn distance(&self, right: &Buf<N>) -> [u8; N] {
        let mut result = [0; N];

        for i in 0..N {
            result[i] = self[i] ^ right[i];
        }

        result
    }
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

pub trait GetKey<Key> {
    fn get_key(&self) -> Key;
}

pub trait GetDirection {
    fn get_direction(&self, i: usize) -> Direction;
}

/// N key size in bits
impl<const N: usize> GetDirection for Buf<N> {
    fn get_direction(&self, i: usize) -> Direction {
        let byte = i >> 3;
        let bit = i % 8;

        // Sanity check
        if byte >= N {
            panic!("Index out of bounds");
        }

        if self[byte] & (1 << (7 - bit)) == 0 {
            Direction::Left
        } else {
            Direction::Right
        }
    }
}

/// Node is either a bucket (it has items) or a fork
struct Node<Item> {
    left: Option<Box<Node<Item>>>,
    right: Option<Box<Node<Item>>>,
    items: Option<Vec<Item>>,
    can_split: bool,
}

impl<Item> Node<Item> {
    pub fn new(can_split: bool) -> Self {
        Node {
            left: None,
            right: None,
            items: Some(Vec::new()),
            can_split
        }
    }
}

/// K: max items in a bucket
struct KBucket<Key, Item, const K: usize> {
    root: Box<Node<Item>>,
    key: Key,
}

/// Key: key struct
/// Item: value struct
/// K: max items in a bucket
impl<const K: usize, Key: PartialEq + GetDirection, Item: GetKey<Key>> KBucket<Key, Item, K> {
    fn new(key: Key) -> Self {
        KBucket {
            key,
            root: Box::new(Node::new(true)),
        }
    }

    pub fn put(&mut self, value: Item) {
        let mut bit_index = 0;
        let mut node = self.root.as_mut();

        let key = value.get_key();

        while node.items.is_none() {
            node = match key.get_direction(bit_index) {
                Direction::Left => node.left.as_mut().unwrap(),
                Direction::Right => node.right.as_mut().unwrap(),
            };

            bit_index += 1;
        }

        // Check if item already exists
        let item_index = match &node.items {
            Some(items) => items.iter().position(|item| item.get_key() == key),
            // This should never happen as we navigated to the leaf node earlier
            None => unreachable!("Node has no items"),
        };

        let items = node.items.as_mut().unwrap();

        // Update item
        if let Some(item_index) = item_index {
            items[item_index] = value;

            return;
        }

        // Insert item
        match items.len() < K {
            // Bucket is not full
            true => items.push(value),
            // Bucket is full, check if we can split
            false => match node.can_split {
                // We can split the node
                true => {
                    // Split node
                    let direction = self.key.get_direction(bit_index);
                    let mut left = Node::new(direction == Direction::Left);
                    let mut right = Node::new(direction == Direction::Right);

                    for item in items.pop() {
                        let key = item.get_key();
                        let direction = key.get_direction(bit_index);

                        match direction {
                            Direction::Left => left.items.as_mut().unwrap().push(item),
                            Direction::Right => right.items.as_mut().unwrap().push(item),
                        }
                    }

                    node.items = None;
                    node.left = Some(Box::new(left));
                    node.right = Some(Box::new(right));

                    // Insert value
                    self.put(value);
                }
                // We cannot split the node, remove the oldest item and insert the new one
                false => {
                    debug!("Bucket is full and cannot be split");
                    items.remove(0);
                    items.push(value);
                }
            },
        }
    }

    pub fn get(&self, key: Key) -> Option<u64> {
        unimplemented!();
    }

    fn update(&self, value: &Item) {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let left = [0, 0, 0, 0, 0, 0, 0, 1];
        let right = [0, 0, 0, 0, 1, 0, 0, 2];

        let result = left.distance(&right);
        assert_eq!(result, [0, 0, 0, 0, 1, 0, 0, 3]);
        assert_eq!(result, right.distance(&left));
    }

    fn test_get_direction() {
        let key = [0, 0, 0, 0, 0, 0, 0, 1];
        assert_eq!(key.get_direction(0), Direction::Right);
        assert_eq!(key.get_direction(1), Direction::Left);
    }
}

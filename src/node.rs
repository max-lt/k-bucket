use std::marker::PhantomData;

use crate::Direction;
use crate::GetDirection;
use crate::GetKey;

/// Node is either a bucket (it has items) or a fork
#[derive(Debug)]
pub(crate) struct Node<Key, Item: GetKey<Key>> {
    pub left: Option<Box<Node<Key, Item>>>,
    pub right: Option<Box<Node<Key, Item>>>,
    pub items: Option<Vec<Item>>,
    pub can_split: bool,

    _marker: PhantomData<Key>,
}

impl<Key: GetDirection, Item: GetKey<Key>> Node<Key, Item> {
    pub fn new_root() -> Self {
        Node {
            can_split: true,
            ..Default::default()
        }
    }

    /// Split the node into two nodes
    pub fn split(&mut self, bit_index: usize, split: Direction) {
        let mut items = self.items.take().unwrap();
        items.reverse();

        let mut left_items = Vec::new();
        let mut right_items = Vec::new();
        while let Some(item) = items.pop() {
            match item.get_key().direction(bit_index) {
                Direction::Left => left_items.push(item),
                Direction::Right => right_items.push(item),
            }
        }

        self.left = Some(Box::new(Node {
            items: Some(left_items),
            can_split: split == Direction::Left,
            ..Default::default()
        }));

        self.right = Some(Box::new(Node {
            items: Some(right_items),
            can_split: split == Direction::Right,
            ..Default::default()
        }));

        self.items = None;
        self.can_split = false;
    }

    /// Get the node that contains the item with the given key
    pub fn get_node<'a>(node: &'a Box<Node<Key, Item>>, key: &Key) -> (&'a Node<Key, Item>, usize) {
        let mut bit_index = 0;
        let mut node = node.as_ref();

        // Navigate to the first bucket node
        while node.items.is_none() {
            node = match key.direction(bit_index) {
                Direction::Left => node.left.as_ref().unwrap(),
                Direction::Right => node.right.as_ref().unwrap(),
            };

            bit_index += 1;
        }

        (node, bit_index)
    }

    /// Get the node that contains the item with the given key (mutable)
    pub fn get_node_mut<'a>(
        node: &'a mut Box<Node<Key, Item>>,
        key: &Key,
    ) -> (&'a mut Node<Key, Item>, usize) {
        let mut bit_index = 0;
        let mut node = node.as_mut();

        // Navigate to the first bucket node
        while node.items.is_none() {
            node = match key.direction(bit_index) {
                Direction::Left => node.left.as_mut().unwrap(),
                Direction::Right => node.right.as_mut().unwrap(),
            };

            bit_index += 1;
        }

        (node, bit_index)
    }

    pub fn count(&self) -> usize {
        match &self.items {
            Some(items) => items.len(),
            None => {
                let left = self.left.as_ref().map(|node| node.count()).unwrap_or(0);
                let right = self.right.as_ref().map(|node| node.count()).unwrap_or(0);

                left + right
            }
        }
    }
}

impl<Key, Item: GetKey<Key>> Default for Node<Key, Item> {
    fn default() -> Self {
        Node {
            left: None,
            right: None,
            items: Some(Vec::new()),
            can_split: true,
            _marker: PhantomData,
        }
    }
}

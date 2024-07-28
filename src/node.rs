use std::marker::PhantomData;

use crate::GetKey;
use crate::HasBitAt;

/// Node is either a bucket (it has items) or a fork
#[derive(Debug)]
pub(crate) struct Node<Key, Item: GetKey<Key>> {
    pub left: Option<Box<Node<Key, Item>>>,
    pub right: Option<Box<Node<Key, Item>>>,
    pub items: Option<Vec<Item>>,
    pub can_split: bool,

    _marker: PhantomData<Key>,
}

impl<Key: HasBitAt, Item: GetKey<Key>> Node<Key, Item> {
    pub fn new_root() -> Self {
        Node {
            can_split: true,
            ..Default::default()
        }
    }

    pub fn split(&mut self, bit_index: usize, next_split_right: bool) {
        let mut items = self.items.take().unwrap();

        let mut left_items = Vec::new();
        let mut right_items = Vec::new();
        while let Some(item) = items.pop() {
            match item.get_key().has_bit_at(bit_index) {
                false => left_items.push(item),
                true => right_items.push(item),
            }
        }

        self.left = Some(Box::new(Node {
            items: Some(left_items),
            can_split: !next_split_right,
            ..Default::default()
        }));

        self.right = Some(Box::new(Node {
            items: Some(right_items),
            can_split: next_split_right,
            ..Default::default()
        }));

        self.items = None;
        self.can_split = false;
    }

    pub fn get_node<'a>(node: &'a Box<Node<Key, Item>>, key: &Key) ->  (&'a Node<Key, Item>, usize) {
        let mut bit_index = 0;
        let mut node = node.as_ref();

        // Navigate to the first bucket node
        while node.items.is_none() {
            node = match key.has_bit_at(bit_index) {
                false => node.left.as_ref().unwrap(),
                true => node.right.as_ref().unwrap(),
            };

            bit_index += 1;
        }

        (node, bit_index)
    }

    pub fn get_node_mut<'a>(node: &'a mut Box<Node<Key, Item>>, key: &Key) ->  (&'a mut Node<Key, Item>, usize) {
        let mut bit_index = 0;
        let mut node = node.as_mut();

        // Navigate to the first bucket node
        while node.items.is_none() {
            node = match key.has_bit_at(bit_index) {
                false => node.left.as_mut().unwrap(),
                true => node.right.as_mut().unwrap(),
            };

            bit_index += 1;
        }

        (node, bit_index)
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

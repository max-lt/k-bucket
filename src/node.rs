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

use std::fmt::Debug;

use crate::Bucket;
use crate::GetKey;
use crate::Node;

trait ToDebugString {
    fn to_debug_string(&self, pos: Vec<u8>, key_bits: usize) -> String;
}

impl<Key, Item: Debug + GetKey<Key>> ToDebugString for Node<Key, Item> {
    fn to_debug_string(&self, pos: Vec<u8>, key_bits: usize) -> String {
        let mut result = String::new();

        let pos_str = pos.iter().map(|x| x.to_string()).collect::<String>();

        if let Some(items) = &self.items {
            result.push_str(&format!(
                "{:>key_bits$} [{}] can_split={}\n",
                pos_str,
                items
                    .iter()
                    .map(|item| format!("{:?}", item))
                    .collect::<Vec<String>>()
                    .join(","),
                self.can_split
            ));
        } else {
            if let Some(left) = &self.left {
                let mut pos = pos.clone();
                pos.push(0);

                result.push_str(&left.to_debug_string(pos, key_bits));
            } else {
                result.push_str(&format!("{pos_str} Left: None\n"));
            }

            if let Some(right) = &self.right {
                let mut pos = pos.clone();
                pos.push(1);
                result.push_str(&right.to_debug_string(pos, key_bits));
            } else {
                result.push_str(&format!("{pos_str} Right: None\n"));
            }
        }

        result
    }
}


impl<Key: Debug, Item: Debug + GetKey<Key>, const K: usize> Debug for Bucket<Key, Item, K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        let key_bits = std::mem::size_of::<Key>() * 8;

        result.push_str(&format!("Bucket({:?}, k={}, n={})\n", self.key, K, key_bits));
        result.push_str(&self.root.to_debug_string(vec![], key_bits));

        write!(f, "{}", result)
    }
}

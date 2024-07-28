

/// Node is either a bucket (it has items) or a fork
#[derive(Debug)]
pub (crate) struct Node<Item> {
    pub (crate) left: Option<Box<Node<Item>>>,
    pub (crate) right: Option<Box<Node<Item>>>,
    pub (crate) items: Option<Vec<Item>>,
    pub (crate) can_split: bool,
}

impl<Item> Node<Item> {
    pub fn new(can_split: bool) -> Self {
        Node {
            left: None,
            right: None,
            items: Some(Vec::new()),
            can_split,
        }
    }

    pub fn new_with_items(items: Vec<Item>, can_split: bool) -> Self {
        Node {
            left: None,
            right: None,
            items: Some(items),
            can_split,
        }
    }
}

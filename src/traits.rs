#[derive(Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

/// [KEY] XOR distance between two keys of same type
pub trait GetDistance {
    fn distance(&self, right: &Self) -> Self;
}

/// [KEY] XOR distance between two keys of same type
pub trait LeadingZeros {
    fn leading_zeros(&self) -> u8;
}

/// [KEY] Returns direction for a given node index i
pub trait GetDirection {
    fn direction(&self, i: usize) -> Direction;
}

/// [ITEM] Returns key of an item
pub trait GetKey<Key> {
    fn get_key(&self) -> Key;
}

/// [ITEM] Arbitrates between two items during an insertion
pub trait Arbiter {
    fn arbitrate(&self, candidate: &Self) -> bool;
}

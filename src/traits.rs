
/// [KEY] XOR distance between two keys of same type
pub trait Distance {
    fn distance(&self, right: &Self) -> Self;
}

/// [KEY] Returns true if bit at index i is set
pub trait HasBitAt {
    fn has_bit_at(&self, i: usize) -> bool;
}

/// [ITEM] Returns key of an item
pub trait GetKey<Key> {
    fn get_key(&self) -> Key;
}
